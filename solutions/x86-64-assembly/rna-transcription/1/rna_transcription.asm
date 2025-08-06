section .text
global to_rna
to_rna:
    cmp rdi, 0
    je .done
    xor rcx, rcx
.loop:
    mov al, [rdi + rcx]
    cmp al, 'A'
    mov byte [rsi + rcx], 'U'
    je .continue
    cmp al, 'C' 
    mov byte [rsi + rcx], 'G'
    je .continue
    cmp al, 'G'
    mov byte [rsi + rcx], 'C'
    je .continue
    cmp al, 'T'
    mov byte [rsi + rcx], 'A'
    je .continue
    jmp .done
.continue:
    inc rcx
    jmp .loop
.done:
    mov byte [rsi + rcx], 0
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif