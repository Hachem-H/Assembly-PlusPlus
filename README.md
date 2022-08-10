# APP
app (pronounced a-p-p, like cpp) or Assembly++, is a scripting language for nasm assembly, providing a better syntax. What is this not? It is not a programming language nor is it an assembler/a compiler. It outputs valid nasm x64-assembly which can then be assembled manually. Think of it like Typescript for JavaScript, but without the JavaScript or the Typescript.

## Example

```assembly
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
```
This basic hello world in x64 nasm can be written as so:
```
message = "Hello World!\n"

proc main 
{
    rax = SYS_WRITE
    rdi = STDOUT
    rsi = message
    rdx = len(message)
    syscall
    
    rax = SYS_EXIT
    rdi = 0x00
    syscall
}
```

Or, since I thought printing stuff out to the console was quite an important feature, we can use the built-in output operator which does require a null terminated string.

```
message = "Hello World!\n\0"

proc main 
{
    rax =  message
    >> rax
    syscall
    
    rax = SYS_EXIT
    rdi = 0x00
    syscall
}
```
