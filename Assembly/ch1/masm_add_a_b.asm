; the addition of a and b in a function
include io32.inc
.i386
.data
    msg db "sum=", 0  
.code
    start:
        call main
        exit 0

        add_a_b proc
            push ebx
            mov eax, [esp + 8]
            mov ebx, [esp + 12]
            add eax, ebx
            pop ebx
        add_a_b endp

        main proc
            push eax
            push 2
            push 3
            call add_a_b
            push eax, offset msg
            dispmsg
            pop eax
            dispuid
            dispcrlf
            pop eax
            ret

        main endp

    end start
