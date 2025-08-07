section .text
global find
find:
    ; rdi = array pointer, esi = size, edx = value
    cmp esi, 0
    je .not_found           ; if size == 0 return -1
    xor eax, eax            ; left = 0
    mov ecx, esi            ; right = size
    dec ecx                 ; right = size - 1
    
.search_loop:
    cmp eax, ecx
    jg .not_found           ; if left > right, not found
    
    ; calculate mid
    mov r8d, eax            ; r8d = left
    add r8d, ecx            ; r8d = left + right
    shr r8d, 1              ; r8d = (left + right) / 2 = mid
    mov r9d, [rdi + r8*4]   ; r9d = arr[mid] (int is 4 bytes)
    
    ; compare array[mid] with value
    cmp r9d, edx
    je .found               ; if arr[mid] == target, return mid
    jl .search_right        ; if arr[mid] < value, search right
    
.search_left:
    mov ecx, r8d            ; right = mid
    dec ecx                 ; right = mid - 1
    jmp .search_loop
    
.search_right:
    mov eax, r8d            ; left = mid
    inc eax                 ; left = mid + 1
    jmp .search_loop
    
.found:
    mov eax, r8d            ; return mid
    ret
    
.not_found:
    mov eax, -1             ; return -1
    ret

%ifidn __OUTPUT_FORMAT__,elf64
section .note.GNU-stack noalloc noexec nowrite progbits
%endif