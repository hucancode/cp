section .text
global square_of_sum
square_of_sum:
    lea eax, [edi+1]    ; eax = n (input)
    imul eax, edi       ; eax *= n
    shr eax, 1          ; eax >> 1
    imul eax, eax       ; eax *= eax
    ret                 ; return pow(n*(n+1)/2, 2)

global sum_of_squares
sum_of_squares:
    lea eax, [edi+1]    ; eax = n (input)
    lea ebx, [edi*2+1]  ; ebx = n (input)
    imul eax, ebx       ; eax *= eax
    imul eax, edi       ; eax *= n
    mov ecx, 6
    xor edx, edx
    div ecx             ; eax /= 6
    ret                 ; return n*(n+1)(2n+1)/6

global difference_of_squares
difference_of_squares:
    lea eax, [edi+1]        ; eax = n+1 (input)
    lea ebx, [edi-1]        ; ebx = n-1 (input)
    lea ecx, [edi*2+edi+2]  ; ecx = n*3+2 (input)
    imul eax, edi           ; eax *= n
    imul eax, ebx           ; eax *= ebx
    imul eax, ecx           ; eax *= ecx
    mov ecx, 12
    xor edx, edx
    div ecx                 ; eax /= 12
    ret                     ; return n(n+1)(n-1)(3n+2)/12

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
