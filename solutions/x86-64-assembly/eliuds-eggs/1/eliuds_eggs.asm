section .text
global egg_count
egg_count:
    mov ebx, edi
    mov eax, 0
.loop:
    cmp ebx, 0
    je .done
    mov ecx, ebx
    and ecx, 1
    cmp ecx, 0
    je .continue
    inc eax
.continue:
    shr ebx, 1
    jmp .loop
.done:
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
