# Assembly++
`a++` (short for assembly plus plus) is a higher level assembly scripting language which transpiles to nasm assembly, it a higher level than raw assembly and lower level than C. It was desgined to make simple tasks in assembly simpler and give a cleaner syntax which is far more readable. This is not an assembler nor a compiler, it's a transpiler. The output of `a++` is not a binary but in fact source code, which can be then be assembled in `nasm` like usual.

**NOTE**: `a++` is still in it's very very early stages and will probably remain in development for quite a long time. Syntax/API changes are almost inevitable, as there is no standard language spec.

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
