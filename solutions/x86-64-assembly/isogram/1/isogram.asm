section .text
global is_isogram
is_isogram:
    xor eax, eax    ; clear character bit mask
.loop:
    mov cl, [rdi]
    cmp cl, 0
    je .true
    or cl, 0x20     ; convert to lowercase
    cmp cl, 'a'
    jb .next
    cmp cl, 'z'
    ja .next
    sub cl, 'a'     ; a-z to 0-25
    bt eax, ecx     ; set bit number ecx (32 bit version of cl)
    jc .false       ; if carry flag set, bit was already 1 means this character has been seen before
    bts eax, ecx
.next:
    inc rdi
    jmp .loop
.false:
    mov eax, 0
    ret
.true:
    mov eax, 1
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
