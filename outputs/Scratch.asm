global someProcedure
global someOtherProc

extern glfwInit
extern glfwTerminate
extern glClear
extern glClearColor
extern printf
extern memcpy

section .text
someProcedure:
    mov rax, message
    mov reserved, rax
    ret

someOtherProc:
    mov rbx, rax
    mov rcx, 165
    mov rdx, 27
    ret

main:
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
    reserved: resb 4
