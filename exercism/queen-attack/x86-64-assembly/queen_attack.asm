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
    ; same row or column
    cmp edi, edx
    je .true
    cmp esi, ecx
    je .true
    ; abs(row diff) in eax
    mov eax, edi
    sub eax, edx
    cdq
    xor eax, edx
    sub eax, edx
    ; abs(col diff) in edx
    mov edx, esi
    sub edx, ecx
    mov ecx, edx
    sar ecx, 31
    xor edx, ecx
    sub edx, ecx
    ; diagonal check
    cmp eax, edx
    je .true
.false:
    mov eax, 0
    ret
.true:
    mov eax, 1
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
