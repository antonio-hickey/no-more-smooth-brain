Chapter 1 Section 4: Symbolic Constants
===

(related to the [tempature conversion program](https://github.com/antonio-hickey/no-more-smooth-brain/blob/master/C/C_Book/chapter_1/section_2/temp_conversion.c) from section 2) It's bad practice to bury "magic numbers" like 300 and 20 in a program; they convey little info to someone and hard to change in a systematic way.

One way to deal with *magic numbers* is to give them meaningful names.

A `#define` line defines a *symbolic name* or *symbolic constant* to be a particular string of characters. 

Example:
    ```C
     #define name replacement text
     ```

    - Thereafter, any occurance of `name` will be replaced with `replacement text`.

* Symbolic Constants are conventionally written in UPPERCASE.

