default rel
section .data
    black: db "black", 0
    brown: db "brown", 0  
    red: db "red", 0
    orange: db "orange", 0
    yellow: db "yellow", 0
    green: db "green", 0
    blue: db "blue", 0
    violet: db "violet", 0
    grey: db "grey", 0
    white: db "white", 0
    bands: dq black, brown, red, orange, yellow, green, blue, violet, grey, white, 0
section .text
global color_code
color_code:
    lea r8, [rel bands]
    mov rax, 0                 ; i = 0
.loop_band:
    mov rcx, [r8 + rax*8]      ; rcx = &bands[i]
    mov rbx, 0                 ; j = 0
    cmp rcx, 0
    je .done
.loop_str:
    mov dl, [rdi+rbx]          ; dl = bands[i][j]
    mov dh, [rcx+rbx]          ; dh = color[j]
    cmp dl, dh
    jne .next_band             ; if el != dl, compare next band
    cmp dl, 0
    je .done                   ; if dl == 0, return
.next_char:
    inc rbx
    jmp .loop_str
.next_band:
    inc rax
    jmp .loop_band
.done:
    ret

global colors
colors:
    lea rax, [rel bands]
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
