section .text
global square_root
square_root:
    cmp edi, 0
    je .return_zero
    mov eax, 1          ; l = 1
    mov ecx, edi        ; r = n
.loop:
    cmp eax, ecx
    jg .found            ; if l > r
    mov edx, eax
    add edx, ecx
    shr edx, 1          ; m = (l+r)/2
    ; test m*m vs n
    mov r8d, edx
    imul r8, r8 
    cmp r8d, edi
    je .found           ; m = n, search no more
    jb .next_right      ; m below n, search right
    ja .next_left       ; m above n, search left
.next_left:
    mov ecx, edx
    dec ecx             ; r = m-1
    jmp .loop
.next_right:
    mov eax, edx
    inc eax             ; l = m+1
    jmp .loop
.found:
    mov eax, edx        ; return m
    ret
.return_zero:
    mov eax, 0
.done:
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
