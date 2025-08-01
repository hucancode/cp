section .data
    earth_seconds_per_year dd 31557600.0
    orbital_periods:
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
    ; edi = planet enum (0-7)
    ; esi = seconds
    cvtsi2ss xmm0, esi
    movss xmm1, [rel earth_seconds_per_year]
    divss xmm0, xmm1 ; seconds /= earth_seconds_per_year
    lea rax, [rel orbital_periods] ; rax = &orbital_periods
    mov edx, edi                ; eax = planet index
    imul edx, 4                 ; eax *= 4 (4 bytes per float)
    movss xmm1, [rax + rdx]  ; xmm1 = orbital_periods[planet]
    divss xmm0, xmm1 ; xmm0 /= xmm1
.done:
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif
