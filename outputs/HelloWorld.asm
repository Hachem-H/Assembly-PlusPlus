main:
    mov rax, 1
    mov rdi, 1
    mov rsi, message
    mov rdx, 14
    syscall

    mov rax, 60
    mov rdi, 0
    syscall

    ret

global _start
section .text
_start:
    call main

section .data
    message: db `Hello World!\n\0`