# General Overview
Program to print a "table" that converts Fahrenheit -> Celsius using
the formula Celsius = (5 / 9) (Fahrenheit - 32).

# Teaching
    - comments
    - declerations
    - variables
    - basic arithmetic expressions
    - while loop
    - formatted output

# Notes
    - comment's work just like JavaScript 
      *(yes I know JavaScript comment's are based off C leave me alone lol)*

      // Comment
      /* Comment */

    - All variables must be declared before use, usually  at the beginning of
      the function before any executable statements.

    - Declerations announces the properties of the variables
        
        `int fahr;` tells the compiler the variable fahr is a integer and 
        treat it as such.

    - Declaring multi variables can be done via comma's
        `int fahr, celsius;` declares `fahr` & `celsius`

    - basic data types in C
        - char (character, a single byte)
        - short (short integer, 2 bytes, range(-32,768 to 32,767))
        - long (short integer, 4 bytes, range(-2,147,483,648 to 2,147,483,647))
        - double (floating point, 8 bytes, range(1.7E-308 to 1.7E+308))

    - formatting strings to print variables
        - %d for formatting a integer into a string

        example (line 15): `printf("%d\t%d\n", fahr, celsius);`  

        - we can also customize the digit width for example
            - 3 digit wide format = `printf("x = %3d", x)` 
            - 3 digit wide Fahrenheit & 6 digit wide celsius:
                `printf("%3d %6d\n", fahr, celsius)`

        - formatting floating point numbers in strings:
            - %f (as floating point)
            - %6f (as floating point, at least 6 chars wide)
            - %.2f (as floating point, 2 chars after decimal point)
            - %6.2f (as floating point, at least 6 chars wide & 2 chars after decimal point)

    - Our conversion arithmetic is not accurate due to
      use of integer arithmetic, for example 0^F != -17^C
      actually 0^F = -17.8^C , so let's use floating point
      arithmetic to get more accurate conversions.

# Excercise's
 - 1 : Modify the program to print a heading above the table
    - Just going to print "Fahrenheit -> Celsius | Conversion Table" at the start of the main f(x)

 - 2 : Write a f(x) to print the Celsius -> Fahrenheit Conversion Table 
    - Formula: Fahrenheit = (Celsius * (9 / 5)) + 32
