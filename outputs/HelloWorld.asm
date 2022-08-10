global _start

; The message is stored in the .data section as it is not going to be overwritten.
; The length is computed, by taking the current address ($) and then subtracting 
; the address of message.

section .data
    message: db "Hello, World!", 0xA
       .len: equ $-message
section .text
_start:
    mov rax, 0x01         ; set rax to the WRITE syscall
    mov rdi, 0x01         ; second argument is the file descriptor, 1 = STDOUT
    mov rsi, message      ; the message
    mov rdx, message.len  ; and the message length
    syscall               ; execute the syscall located in rax
    
    mov rax, 0x3C         ; set rax to the EXIT syscall
    mov rdi, 0x00         ; second aregument is the return value, 0 = success
    syscall               ; same jargon
