section .rodata
    seconds_per_year dd 31557600.0
    multipliers:
        dd 0.2408467    ; MERCURY
        dd 0.61519726   ; VENUS
        dd 1.0          ; EARTH
        dd 1.8808158    ; MARS
        dd 11.862615    ; JUPITER
        dd 29.447498    ; SATURN
        dd 84.016846    ; URANUS
        dd 164.79132    ; NEPTUNE
section .text
global age
age:
    ; edi = planet index (0-7)
    ; esi = seconds
    cvtsi2ss xmm0, esi                 ; xmn0 = signed_int_to_float(esi)
    divss xmm0, [rel seconds_per_year] ; seconds /= seconds_per_year
    lea rax, [rel multipliers]         ; rax = &multipliers
    mov edx, edi                       ; edx = planet index
    imul edx, 4                        ; edx *= 4 (4 bytes per float)
    divss xmm0, [rax + rdx]            ; xmm0 /= xmm1
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
