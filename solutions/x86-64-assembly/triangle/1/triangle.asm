section .text

global is_equilateral
is_equilateral:
    movsd xmm0, [rsp+8]   ; a
    movsd xmm1, [rsp+24]  ; b  
    movsd xmm2, [rsp+16]  ; c
    
    xorpd xmm3, xmm3      ; 0.0
    ucomisd xmm0, xmm3
    jbe .false
    ucomisd xmm1, xmm3  
    jbe .false
    ucomisd xmm2, xmm3
    jbe .false
    
    ucomisd xmm0, xmm1
    jp .false
    jne .false
    
    ucomisd xmm0, xmm2
    jp .false
    jne .false
    
    mov eax, 1
    ret
    
.false:
    xor eax, eax
    ret

global is_isosceles
is_isosceles:
    movsd xmm0, [rsp+8]   ; a
    movsd xmm1, [rsp+24]  ; b  
    movsd xmm2, [rsp+16]  ; c
    
    xorpd xmm3, xmm3
    ucomisd xmm0, xmm3
    jbe .false
    ucomisd xmm1, xmm3  
    jbe .false
    ucomisd xmm2, xmm3
    jbe .false
    
    ; Triangle inequality
    movapd xmm4, xmm0
    addsd xmm4, xmm1
    ucomisd xmm4, xmm2
    jbe .false
    
    movapd xmm4, xmm0
    addsd xmm4, xmm2
    ucomisd xmm4, xmm1
    jbe .false
    
    movapd xmm4, xmm1
    addsd xmm4, xmm2
    ucomisd xmm4, xmm0
    jbe .false
    
    ; Check any two sides equal
    ucomisd xmm0, xmm1
    jp .check_ac
    je .true
    
.check_ac:
    ucomisd xmm0, xmm2
    jp .check_bc
    je .true
    
.check_bc:
    ucomisd xmm1, xmm2
    jp .false
    je .true
    
.false:
    xor eax, eax
    ret
    
.true:
    mov eax, 1
    ret

global is_scalene
is_scalene:
    movsd xmm0, [rsp+8]   ; a
    movsd xmm1, [rsp+24]  ; b  
    movsd xmm2, [rsp+16]  ; c
    
    xorpd xmm3, xmm3
    ucomisd xmm0, xmm3
    jbe .false
    ucomisd xmm1, xmm3  
    jbe .false
    ucomisd xmm2, xmm3
    jbe .false
    
    ; Triangle inequality
    movapd xmm4, xmm0
    addsd xmm4, xmm1
    ucomisd xmm4, xmm2
    jbe .false
    
    movapd xmm4, xmm0
    addsd xmm4, xmm2
    ucomisd xmm4, xmm1
    jbe .false
    
    movapd xmm4, xmm1
    addsd xmm4, xmm2
    ucomisd xmm4, xmm0
    jbe .false
    
    ; Check no sides equal
    ucomisd xmm0, xmm1
    jp .check_ac
    je .false
    
.check_ac:
    ucomisd xmm0, xmm2
    jp .check_bc
    je .false
    
.check_bc:
    ucomisd xmm1, xmm2
    jp .true
    je .false
    
.true:
    mov eax, 1
    ret
    
.false:
    xor eax, eax
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif