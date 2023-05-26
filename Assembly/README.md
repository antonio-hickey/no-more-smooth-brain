Assembly Language (ASM)
===

Key Concepts
---
* `Assembler`: creates object code by translating combinations of mnemonics and syntax for operations and addressing modes into their numerical equivalents. This representation typically includes an operation code ("opcode") as well as other control bits and data. The assembler also calculates constant expressions and resolves symbolic names for memory locations and other entities. The use of symbolic references is a key feature of assemblers, saving tedious calculations and manual address updates after program modifications. Most assemblers also include macro facilities for performing textual substitution – e.g., to generate common short sequences of instructions as inline, instead of called subroutines.
    * One-pass assemblers: process the source code once.  For symbols used before they are defined, the assembler will emit "errata" after the eventual definition, telling the linker or the loader to patch the locations where the as yet undefined symbols had been used.
    * Multi-pass assemblers: on the first pass create a table with all symbols and their values, then use the table in future passes to generate code.
    * High-level assemblers: more sophisticated assemblers that provide language abstractions like:
        * High-level procedure/function declarations and invocations
        * Advanced control structures (IF/THEN/ELSE, SWITCH)
        * High-level abstract data types, including structures/records, unions, classes, and sets
        * Sophisticated macro processing 
        * Object-oriented programming features such as classes, objects, abstraction, polymorphism, and inheritance
* Assembly Language: Consists of a series of mnemonic processor instructions and meta-statements (known variously as declarative operations, directives, pseudo-instructions, pseudo-operations and pseudo-ops), comments and data. Assembly language instructions usually consist of an opcode mnemonic followed by an operand, which might be a list of data, arguments or parameters. Some instructions may be "implied," which means the data upon which the instruction operates is implicitly defined by the instruction itself—such an instruction does not take an operand. The resulting statement is translated by an assembler into machine language instructions that can be loaded into memory and executed.
    * Example: The instruction below tells an x86 processor to move an immediate 8-bit value into a register.
    ```asm
        MOV AL, 61h     ; Load AL with 97 decimal (61 hex)
    ```

Language Design
---

* Basic elements:
    * Opcode mnemonics:
        * To make instructions (statements) in assembly very simple it uses symbolic name for `opcodes`
            * Example: the mnemonic `MOV` is a symbolic name for the opcode for moving values.
        * Extended mnemonics are used to support specialized uses of instructions.
            * Example: many CPUs do not have an explicit NOP instruction, but have instructions that can be used for that purpose.
                * 8086 CPUs the instruction `xchg ax,ax` is used for `nop`, with `nop` being pseudo-opcode to encode the instruction `xchg ax,ax`.
        * Mnemonics are basically just arbitary symbols for opcodes.
    
    * Data directives: There are instructions used to define data elements to hold data and variables. They define the type, length, and alignment of data. They also define whether the data is available for use outside the program or only to the program in which it's defined.
    
    * Assembly directives: commands given to an assembler "directing it to perform operations other than assembling instructions". Affecting how the assembler operates and "may affect the object code, the symbol table, the listing file, and the values of internal assembler parameters". Sometimes the term pseudo-opcode is reserved for directives that generate object code, such as those that generate data.

* Macros:
    * In assembly language, the term "macro" represents a more comprehensive concept than it does in some other contexts, such as the pre-processor in the C programming language, where its #define directive typically is used to create short single line macros. Assembler macro instructions, like macros in PL/I and some other languages, can be lengthy "programs" by themselves, executed by interpretation by the assembler during assembly. 
    * Since macros can have 'short' names but expand to several or indeed many lines of code, they can be used to make assembly language programs appear to be far shorter, requiring fewer lines of source code, as with higher level languages. They can also be used to add higher levels of structure to assembly programs, optionally introduce embedded debugging code via parameters and other similar features. 

