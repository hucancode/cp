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
    cmp byte [rax+1], 0    ; if *(rax+1) == NULL
    je .loop               ; break
    inc rax                ; rax++
    jmp .find_terminal     ; continue
.loop:
    cmp rsi, rax           ; if rsi >= rax
    jge .done              ; finish
    mov cl, [rsi]          ; cl = *rsi
    mov ch, [rax]          ; ch = *rax
    mov [rsi], ch          ; *rsi = ch
    mov [rax], cl          ; *rax = cl
    inc rsi                ; rsi++
    dec rax                ; rax--
    jmp .loop              ; continue
.done:
    ret
%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
