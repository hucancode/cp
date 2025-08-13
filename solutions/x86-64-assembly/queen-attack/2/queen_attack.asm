section .text
global can_create
can_create:
    cmp edi, 0
    jl .false
    cmp edi, 7
    jg .false
    cmp esi, 0
    jl .false
    cmp esi, 7
    jg .false
    mov eax, 1
    ret
.false:
    mov eax, 0
    ret
global can_attack
can_attack:
    ; same row or same column
    cmp edi, edx
    je  .true
    cmp esi, ecx
    je  .true
    ; row_diff = edi - edx
    mov eax, edi
    sub eax, edx
    ; col_diff = esi - ecx
    mov edx, esi
    sub edx, ecx
    ; diagonal check: row_diff == col_diff  OR row_diff == -col_diff
    cmp eax, edx
    je  .true
    neg edx
    cmp eax, edx
    je  .true
.false:
    xor eax, eax
    ret
.true:
    mov eax, 1
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
