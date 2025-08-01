section .text
global steps
steps:
    mov eax, edi          ; Move first argument to RAX
    mov ecx, -1           ; Step counter = -1
    cmp eax, 0            ; Compare n with 0
    jle .done             ; If less than or equal 0, exit
.loop:
    inc ecx               ; ret++
    cmp eax, 1            ; Compare n with 1
    je .done              ; If equal, exit loop
    test eax, 1           ; Check if n is even (n & 1 == 0)
    jnz .odd_case         ; If odd, jump to odd case
    shr eax, 1            ; n /= 2
    jmp .loop             ; Continue loop
.odd_case:
    mov ebx, 3            ; k = 3
    mul ebx               ; n *= k
    inc eax               ; n++
    jmp .loop             ; Continue loop
.done:
    mov eax, ecx          ; Return ret (store in RAX)
    ret                   ; Return to caller

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
