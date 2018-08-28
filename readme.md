# Simple Brainfuck Interpreter
Simple Brainfuck Interpreter written in Rust for learning purposes.
## Build
To build you rust toolchain and cargo installed. Then Run this command in root directory
```
cargo build
```
## Usage
To use run this command on compiled executable and bbrain fuck script ex.
```
./simple-brainfuck.exe example.bf
```
### About Brainfuck
Brainfuck is an esoteric programming language created in 1993 by Urban Müller, and notable for its extreme minimalism.

The language consists of only eight simple commands and an instruction pointer. While it is fully Turing complete, it is not intended for practical use, but to challenge and amuse programmers. Brainfuck simply requires one to break commands into microscopic steps.

The language's name is a reference to the slang term brainfuck, which refers to things so complicated or unusual that they exceed the limits of one's understanding.

These instructions

<table>
    <thead>
            <tr>
            <th align="center">Character</th>
            <th align="center">Meaning</th>
        </tr>
    </thead>
    <tbody>
    <tr>
        <td align="center"> > </td>
        <td align="center">increment the data pointer (to point to the next cell to the right).</td>
    </tr>
    <tr>
        <td align="center"> < </td>
        <td align="center">decrement the data pointer (to point to the next cell to the left).</td>
    </tr>
    <tr>
        <td align="center"> + </td>
        <td align="center">increment (increase by one) the byte at the data pointer.</td>
    </tr>
    <tr>
        <td align="center"> - </td>
        <td align="center">	decrement (decrease by one) the byte at the data pointer.</td>
    </tr>
    <tr>
        <td align="center"> . </td>
        <td align="center">output the byte at the data pointer.</td>
    </tr>
    <tr>
        <td align="center"> , </td>
        <td align="center">accept one byte of input, storing its value in the byte at the data pointer.</td>
    </tr>
    <tr>
        <td align="center"> [ </td>
        <td align="center">if the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.</td>
    </tr>
    <tr>
        <td align="center"> ] </td>
        <td align="center">if the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.</td>
    </tr>
    </tbody>
</table>

[Source](https://en.wikipedia.org/wiki/Brainfuck) : wikipedia.org