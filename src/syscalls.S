format ELF

include "src/func_constants.inc"

section '.text'
    public _exit
    public _start_window_draw
    public _end_window_draw
    public _define_window
    public _display_message
    public _wait_event
    public _pressed_key

_exit:
    mov eax, SF_TERMINATE_PROCESS
    int 0x40

_start_window_draw:
    mov eax, SF_REDRAW
    mov ebx, SSF_BEGIN_DRAW
    int 0x40
    ret

_end_window_draw:
    mov eax, SF_REDRAW
    mov ebx, SSF_END_DRAW
    int 0x40
    ret

_define_window:
    push edi
    push esi
    mov eax, SF_CREATE_WINDOW
    mov ebx, dword [esp + 0x0c]
    mov ecx, dword [esp + 0x10]
    mov edx, dword [esp + 0x14]
    mov esi, dword [esp + 0x18]
    mov edi, dword [esp + 0x1c]
    int 0x40
    pop esi
    pop edi
    ret

_display_message:
    push edi
    push esi
    mov eax, SF_DRAW_TEXT
    mov ebx, dword [esp + 0x0c]
    mov ecx, dword [esp + 0x10]
    mov edx, dword [esp + 0x14]
    mov esi, dword [esp + 0x18]
    mov edi, dword [esp + 0x1c]
    int 0x40
    pop esi
    pop edi
    ret

_wait_event:
    mov eax, SF_WAIT_EVENT
    int 0x40
    ret

_pressed_key:
    mov eax, SF_GET_KEY
    int 0x40
    ret
