global  _main
global _add_a_and_b
extern _printf
section .data
    msg: db "sum=%d", 13, 10, 0
section .bss
    
section .text
    _add_a_and_b:
        push ebx
        mov eax, [esp+8]
        mov ebx, [esp+12]
        add eax, ebx
        pop ebx
        ret

    _main:
        push 3
        push 2
        call _add_a_and_b
        add esp, 8
        push eax
        push msg
        call _printf
        add esp, 8
        ret

;; nasm -fwin32 f.asm -o f.o
;; gcc f.o -o f
;; os: win