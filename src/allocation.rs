use crate::system::debug_write;
use crate::{sys, throw_new};
use core::alloc::Layout;
use core::mem::size_of;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicBool, Ordering};

extern crate alloc;

const PAGE_SIZE: usize = 4096;
static mut MAIN_SECTOR: usize = 0;

#[derive(Clone, Copy)]
enum Sign {
    Dead = 0,
    Active = 1,
    Free = 2,
}

#[derive(Clone, Copy)]
struct SectorHeader {
    pub size: usize,
    pub size_left: usize,
}

#[derive(Clone, Copy)]
struct BlockHeader {
    pub sign: Sign,
    pub size: usize,
}

static HEAP_INIT: AtomicBool = AtomicBool::new(false);

pub fn init() {
    if !HEAP_INIT.swap(true, Ordering::Relaxed) {
        unsafe {
            sys::init_heap();
            MAIN_SECTOR = sys::alloc(PAGE_SIZE) as usize;
        }
    }
}

pub fn malloc(size: usize) -> *mut u8 {
    unsafe {
        for i in 0..PAGE_SIZE / 4 {
            let addr = *((MAIN_SECTOR + i * 4) as *const u32) as *const u8;

            if (addr as usize) != 0 {
                let sec = addr;
                let mut hdr = *(addr as *const SectorHeader);
                let sec_start_blocks = (sec as usize) + size_of::<SectorHeader>();
                if hdr.size_left >= size {
                    let mut j = sec_start_blocks;
                    let mut first_found_block_addr = 0;

                    while j <= sec_start_blocks + hdr.size {
                        let mut block = *(j as *const BlockHeader);
                        match block.sign {
                            Sign::Active => {
                                // If block is occupated - pass
                                first_found_block_addr = 0;
                                j += size_of::<BlockHeader>() + block.size;
                            }

                            Sign::Free => {
                                if first_found_block_addr != 0 {
                                    first_found_block_addr = j;
                                }

                                let sum_size = j - first_found_block_addr + block.size;
                                if sum_size < size {
                                    // if not enough size - pass and find next block
                                    j += (size_of::<BlockHeader>()) + block.size;
                                } else if size - sum_size < size_of::<BlockHeader>() {
                                    // Create 2 blocks
                                    let mut main_block =
                                        *(first_found_block_addr as *const BlockHeader);
                                    main_block.sign = Sign::Active;
                                    main_block.size = size;

                                    let mut secondary_block = *(first_found_block_addr
                                        as *const BlockHeader)
                                        .add(size_of::<BlockHeader>() + size);
                                    secondary_block.sign = Sign::Free;
                                    secondary_block.size =
                                        sum_size - size - size_of::<BlockHeader>();

                                    return (first_found_block_addr as *mut u8)
                                        .add(size_of::<BlockHeader>());
                                } else {
                                    // Create 1 block
                                    let mut main_block =
                                        *(first_found_block_addr as *const BlockHeader);
                                    main_block.sign = Sign::Active;
                                    main_block.size = sum_size - size_of::<BlockHeader>();
                                    return (first_found_block_addr as *mut u8)
                                        .add(size_of::<BlockHeader>());
                                }
                            }

                            Sign::Dead => {
                                // We found \0 - dead zone. There are no further blocks
                                if j + size + size_of::<BlockHeader>()
                                    <= sec_start_blocks + hdr.size
                                {
                                    // There is enough space for creating new block
                                    block.sign = Sign::Active;
                                    block.size = size;
                                    hdr.size_left -= size + size_of::<BlockHeader>();
                                    return (j + size_of::<BlockHeader>()) as *mut u8;
                                } else {
                                    // There is not enough space, go to next sector
                                    break;
                                }
                            }
                        }
                    }
                }
            } else {
                let sec_size = size + PAGE_SIZE - size % PAGE_SIZE;
                let new_sec = sys::alloc(sec_size);
                let sec_hdr = new_sec as *mut SectorHeader;
                *sec_hdr = SectorHeader {
                    size: sec_size,
                    size_left: sec_size - size_of::<SectorHeader>(),
                };
                let new_block = new_sec.add(size_of::<SectorHeader>()) as *mut BlockHeader;
                (*new_block).sign = Sign::Active;
                (*new_block).size = size;
                (*sec_hdr).size_left -= size + size_of::<BlockHeader>();
                return new_block.add(1) as *mut u8;
            }
        }
    }

    panic!("Malloc error: end of the loop")
}

fn free(block: *const u8) {
    unsafe {
        let mut block_hdr = *(block.sub(size_of::<BlockHeader>()) as *mut BlockHeader);

        for i in 0..PAGE_SIZE / 4 {
            let addr = *((MAIN_SECTOR + i * 4) as *const u32) as *const u8;
            let mut hdr = *(addr as *const SectorHeader);

            if addr < block && (block as usize) < (addr as usize) + hdr.size {
                hdr.size_left += block_hdr.size;
                if hdr.size_left == hdr.size - size_of::<SectorHeader>() {
                    free(addr)
                } else {
                    block_hdr.sign = Sign::Free;
                }
                break;
            }
        }

        if !sys::free(block) {
            panic!("Free failed");
        }
    }
}

struct GlobalAlloc;

unsafe impl alloc::alloc::GlobalAlloc for GlobalAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() != 1 {
            throw_new!("Only byte aligned available now");
            return null_mut();
        }

        init();
        malloc(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        // free keeps track of layout presumably????
        free(ptr)
    }
}

#[global_allocator]
static ALLOC: GlobalAlloc = GlobalAlloc;
