# CPSC 121 Decompiler

So, fellow student, are you tired of doing that last CPSC 121 lab by hand? Do
you yearn to understand the assembler code? Well lucky for you I have just the
solution!

Enter the `cpsc121-decom` assembler decompiler program! It helps by decoding the
cryptic hex codes into a (more) readable assembler language!

```
30 f2 0 0 0 1 30 f1
0 0 0 5 30 f0 0 0 0 0 62
11 71 0 0 0 22 60 10
61 21 70 0 0 0 12 0
```

into...

```
00: MOV rf & r2 <= 00000001
06: MOV rf & r1 <= 00000005
0c: MOV rf & r0 <= 00000000
12: AND r1, r1
14: JLE 00000022
19: ADD r1, r0
1b: SUB r2, r1
1d: JMP 00000012
22: HALT
```

Here is the language that I use:

## HALT (0x00)

Halts the computer.

## MOV (0x30)

Moves a hex value into 2 registers. If any of the registers are named "f", then
the move is ignored. The 6 bytes are laid out as the following:

```
2 bytes (0x30)  instruction code
half-byte       first register name (rA)
half-byte       second register name (rB)
4 bytes         data to put in (value)
```

It is equivalent to the following code:

```c++
unsigned rA, rB, value;
rA = value;
rB = value;
```

It is written as follows:

```
MOV rA & rB <= value
```

## ADD (0x60), SUB (0x61), and AND (0x62)

Performs the specified arithmatic operation on the two registers, and stores the
result in the second register. The 2 bytes are laid out as the following:

```
1 byte (0x6Y)   instruction code
half-byte       first register name (rA)
half-byte       second register name (rB)
```

It is equivalent to the following code:

```c++
unsigned rA, rB;

// Addition
rB += rA;
// Subtraction
rB -= rA;
// ANDtion
rB &= rA;
```

It is written as follows:

```
ADD rA, rB
SUB rA, rB
AND rA, rB
```

## JMP (0x70), and JLE (0x71)

Performs either an unconditional jump or a jump-if-less-than operation. The JLE
operation checks to see if the last arithmatic operation results in the result
being less than or equal to zero, and if so, jumps to the specified address. The
5 bytes are laid out as the following:

```
2 bytes (0x7Y)  instruction code
4 bytes         absolute address to jump to (addr)
```

It is equivalent to the following code:

```c++
// Unconditional
loop:
doThing();
goto loop;

// Jump if less or equal to
unsigned val;
loop:
doThing();
if (val <= 0) goto loop;
```

It is written as follows:

```
JMP addr
JLE addr
```
