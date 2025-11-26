section .text
global distance
distance:
    ; rdi = &str1, rsi = &str2
    xor eax, eax  ; ret = 0
    xor ebx, ebx  ; i = 0

.loop:
    mov cl, [rdi + rbx]     ; cl = str1[i]
    mov ch, [rsi + rbx]     ; ch = str2[i]
    cmp cl, 0
    je .str1_ended          ; if cl == 0 check string length
    cmp ch, 0               ; if ch == 0 return -1
    je .bad_length
    cmp cl, ch
    je .continue            ; if cl == ch continue
    inc eax                 ; ret++

.continue:
    inc rbx                 ; i++
    jmp .loop

.str1_ended:
    cmp ch, 0
    jne .bad_length         ; if ch != 0 return -1
    ret

.bad_length:
    mov eax, -1
    ret                     ; return -1

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
