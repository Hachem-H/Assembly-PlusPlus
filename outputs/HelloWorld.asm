section .text
main:
    mov rax, 1
    mov rdi, 1
    mov rsi, message
    mov rdx, 13
    syscall

    mov rax, 60
    mov rdi, 0
    syscall

    ret

global _start
_start:
    call main
section .data
    message: db `Hello World!\n`
section .bss
