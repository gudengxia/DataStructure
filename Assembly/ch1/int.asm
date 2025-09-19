;; nasm -f elf _f.asm -o _f.o
;; ld -s -m elf_i386 _f.o -o f (the entry lable is _start)
section .text
global _start
_start:
   ;; eax = 4, ebx = 1, ecx = output, edx = length
   mov  eax,4		;sys_wite
   mov  ebx,1		;To stdout
   mov  ecx,msg		;'Input some data: '
   mov  edx,msg_size	
   int  80h		;Call kernel

   ;; eax = 4, ebx = 1, ecx = output, edx = length
   mov  eax,3		;sys_read. Read what user inputs
   mov  ebx,0		;From stdin
   mov  ecx,inp_buf	;Save user input to buffer.
   mov  edx, 256
   int    80h

   push eax

   mov  eax,4
   mov  ebx,1
   mov  ecx,msg2		;'You entered: '
   mov  edx,msg2_size
   int    80h

   mov  eax,4		
   mov  ebx,1	
   mov  ecx,inp_buf
   pop  edx
   int  80h

   mov  eax,1
   mov  ebx,0
   int  80h

section .bss
   inp_buf resb 256

section .data
   msg: db 'Input some data: '
   msg_size: equ $-msg

   msg2: db 'You entered: '
   msg2_size: equ $-msg2
