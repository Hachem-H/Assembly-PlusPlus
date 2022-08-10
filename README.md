# APP
`app` (a.p.p., short for assembly plus plus) is a higher level assembly scripting language which transpiles to nasm assembly, it a higher level than raw assembly and lower level than C. It was desgined to make simple tasks in assembly simpler and give a cleaner syntax which is far more readable. This is not an assembler nor a compiler, it's a transpiler. The output of `app` is not a binary but in fact source code, which can be then be assembled in `nasm` like usual.

**NOTE**: `app` is still in it's very very early stages and will probably remain in development for quite a long time. Syntax/API changes are almost inevitable, as there is no standard language spec.

## Example
The program we all start with of course is a basic `Hello World!`, which in x64 nasm looks quite scary.
```assembly
global _start

section .data
    message: db "Hello, World!", 0xA
       .len: equ $-message
; The message is stored in the .data section as it is not going to be overwritten.
; The length is computed, by taking the current address ($) and then subtracting 
; the address of message.

section .text
_start:
    mov rax, 0x01         ; set rax to the WRITE syscall
    mov rdi, 0x01         ; second argument is the file descriptor, 1 = STDOUT
    mov rsi, message      ; the message
    mov rdx, message.len  ; and the message length
    syscall               ; execute the syscall located in rax
    
    mov rax, 0x3C         ; set rax to the EXIT syscall
    mov rdi, 0x00         ; second aregument is the return value, 0 = success
    syscall               ' same jargon
```
The same example can be written in `app` using a fairly remarkably close and cleaner syntax.
```
message = "Hello, World!\n"
; Allocate the memory for the message, since it's intialized,
; `app` knows to put this in the .data section. Otherwise,
; it would reserve a buffer in .bss

proc main  ; _start is replaced with main
{
    rax = SYS_WRITE      ; we no longer need a table since the syscalls are predefined as constants
    rdi = STDOUT         ; same here with the file descriptor
    rsi = message
    rdx = len(message)   ; the length is computed as compile time, its a special location in memory
    syscall
    
    rax = SYS_EXIT       ; same jazz over here too
    rdi = 0x00
    syscall
}
```

## Usage
Running `--help` without any other argument shows the following usage prompt.
```
OPTIONS:
    -h, --help
            Print help information

    -o, --output <OUTPUT_FILE>
            [default: output.asm]

    -s, --source <SOURCE_FILE>
```
Example:
```
$ app -s SuperImportantScript.a++ -o AsmScary.asm
$ app -s NASA-Hack.a++ # output in output.asm
```
