main:
<<<<<<< HEAD
    mov rax, 1
    mov rdi, 1
    mov rsi, message
    mov rdx, 14
    syscall
    mov rax, 60
    mov rdi, 0
    syscall
    ret
=======
mov rax, 1
mov rdi, 1
mov rsi, message
mov rdx, 14
syscall

mov rax, 60
mov rdi, 0
syscall

ret
>>>>>>> 1f18a9fe20cee215fd11496e516b6ef9a1d0e527

global _start
section .text
_start:
    call main
<<<<<<< HEAD

section .data
    message: db `Hello World!\n\0`
=======
section .data
message: db `Hello World!\n\0`
>>>>>>> 1f18a9fe20cee215fd11496e516b6ef9a1d0e527
