section .text
main:
    mov rax, message
    mov reserved, rax
    mov rax, 60
    syscall

    ret

global _start
_start:
    call main

section .data
    message: db `Hello World!\n`
section .bss
    reserved: resb 4
