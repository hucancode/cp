section .text
global leap_year
leap_year:
    mov eax, edi          ; Move first argument to EAX
    mov ecx, 400          ; k = 400
    mov edx, 0            ; Clear EDX
    div ecx               ; n /= k, remainder r is in edx
    cmp edx, 0            ; compare r with 0
    je .true              ; if n%400 == 0, this is a leap year

    mov eax, edi          ; Move first argument to EAX
    mov ecx, 100          ; k = 100
    mov edx, 0            ; Clear EDX
    div ecx               ; n /= k, remainder r is in edx
    cmp edx, 0            ; compare r with 0
    je .false             ; if n%100 == 0, this is not a leap year

    mov eax, edi          ; Move first argument to EAX
    mov ecx, 4            ; k = 4
    mov edx, 0            ; Clear EDX
    div ecx               ; n /= k, remainder r is in edx
    cmp edx, 0            ; compare r with 0
    je .true              ; if n%4 == 0, this is a leap year

    jmp .false            ; otherwise, this is not a leap year

.true:
    mov eax, 1
    jmp .done

.false:
    mov eax, 0
    jmp .done

.done:
    ret                   ; Return to caller

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
