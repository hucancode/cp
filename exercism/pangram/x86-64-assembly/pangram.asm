section .text
global is_pangram
is_pangram:
    xor eax, eax   ; clear character bit mask
.loop:
    mov cl, [rdi]
    cmp cl, 0
    je .check
    or cl, 0x20    ; convert to lowercase
    cmp cl, 'a'
    jb .next
    cmp cl, 'z'
    ja .next
    sub cl, 'a'    ; a-z to 0-25
    bts eax, ecx   ; Set bit at position ecx
.next:
    inc rdi
    jmp .loop
.check:
    cmp eax, 0x3FFFFFF ; f = 0b1111, 3 = 0b11
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
