section .text
global square
square:
    mov cl, dil          ; Move first argument to CL
    cmp cl, 0            ; Compare n with 0
    jle .exception       ; Invalid input, exit
    cmp cl, 65           ; Compare n with 65
    jge .exception       ; Invalid input, exit
    dec cl               ; Decrement CL
    mov rax, 1           ; Set RBX to 1
    shl rax, cl          ; Shift left by n
    ret
.exception:
    mov rax, 0           ; Set RAX to 0
    ret

global total
total:
    mov rax, 0           ; Set RAX to 0
    dec rax              ; Decrement RAX
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
