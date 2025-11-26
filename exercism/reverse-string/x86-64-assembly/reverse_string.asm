section .text
global reverse
reverse:
    cmp rdi, 0             ; if str == NULL
    je .done               ; do nothing
    cmp byte [rdi], 0      ; if str[0] == NULL
    je .done               ; do nothing
    mov rsi, rdi           ; rsi = &str
    mov rax, rdi           ; rax = &str
.find_terminal:
    cmp byte [rax+1], 0
    je .loop
    inc rax
    jmp .find_terminal
.loop:
    cmp rsi, rax           ; if rsi == rax
    jge .done              ; finish
    mov cl, [rsi]          ; cl = str[rsi]
    mov ch, [rax]          ; ch = str[rax]
    mov [rsi], ch          ; str[rsi] = ch
    mov [rax], cl          ; str[rax] = cl
    inc rsi
    dec rax
    jmp .loop
.done:
    ret
%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
