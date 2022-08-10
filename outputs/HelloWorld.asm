
global _start

section .data
    message: db "Hello World!", 0xA
       .len: equ $-message

section .text
_start:
    mov rax, 0x01
    mov rdi, 0x01
    mov rsi, message
    mov rdx, message.len
    syscall
    
    mov rax, 0x3C
    mov rdi, 0x00
    syscall  
