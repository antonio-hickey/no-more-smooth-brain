Chapter 1 Section 5: Character Input and Output
===

The model of input and output supported by the standard library is very simple. Text input or output is dealt with as streams of characters.

A *text stream* is a sequence of characters divided into lines; each line consists of zero or more characters followed by a new line character.

The standard library provides serveral f(x)'s for reading or writing one character at a time.
    - `getchar` : reads the *next* input character from a text stream and returns it's value.
    - `putchar` : prints a character each time it's called.


File Copying
---
Given getchar & putchar, we can write a program to copy it's input into it's output one character at a time.
