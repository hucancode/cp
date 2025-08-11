section .rodata
    ring0: dq 1.0
    ring1: dq 25.0
    ring2: dq 100.0
section .text
global score
score:
    mulsd xmm0, xmm0 ; a *= a
    mulsd xmm1, xmm1 ; b *= b
    addsd xmm0, xmm1 ; a += b
    ucomisd xmm0, [rel ring0]
    jbe .ring0
    ucomisd xmm0, [rel ring1]
    jbe .ring1
    ucomisd xmm0, [rel ring2]
    jbe .ring2
    mov al, 0
    ret
.ring0:
    mov al, 10
    ret
.ring1:
    mov al, 5
    ret
.ring2:
    mov al, 1
    ret
%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
