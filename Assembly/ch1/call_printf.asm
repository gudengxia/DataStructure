; use 32-bit gcc compiler that is in CodeBlocks
; nasm -f win32 -o hello.o
; gcc hello.o -o hello.exe
global  _main
extern  _printf
section .text
    _main:
        push    message
        call    _printf
        add     esp, 4
        ret
section .data
message:
        db      'Hello, World', 10, 0