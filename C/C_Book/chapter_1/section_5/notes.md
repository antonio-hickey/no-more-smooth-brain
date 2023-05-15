Chapter 1 Section 5: Character Input and Output
===

The model of input and output supported by the standard library is very simple. Text input or output is dealt with as streams of characters.

A *text stream* is a sequence of characters divided into lines; each line consists of zero or more characters followed by a new line character.

The standard library provides serveral f(x)'s for reading or writing one character at a time.
    - `getchar` : reads the *next* input character from a text stream and returns it's value.
    - `putchar` : prints a character each time it's called.


File Copying (1.5.1)
---
Given getchar & putchar, we can write a program to copy it's input into it's output one character at a time.

A better way to write this would be to skip the assignment of `C`, and just `getchar()` in the test part of the
while loop. This way centralizes the input there, and is now only one refrence to `getchar` shrinking the program. 

Excercise 1-7: print the value of EOF:
    *  just call `printf("%d", EOF)` , using "%d" becuase `EOF` is a integer 0 or 1.


Character Counting (1.5.2)
---
Just introducing using `++` operator on a variable through a iterator to get a count.

