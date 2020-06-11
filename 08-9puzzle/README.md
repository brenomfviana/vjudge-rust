# Puzzle 9

## Description

Alex has a puzzle her father gave her last Christmas. It has nine numbered
squares arranged in a 3 × 3 matrix (three rows and three columns) and it's
mechanically designed to allow the following types of movements:

- A horizontal right move shifts one position to the right each of the squares
  in the corresponding horizontal row (circularly).
- A vertical up move shifts one position upwards each of the squares in the
  corresponding vertical column (circularly).

Alex's troublemaker little brother Jim snuck last night into his sister's
bedroom and somehow tore the puzzle apart and put it back together. However,
when Jim assembled the puzzle back, he might have done it in a configuration
different from the original configuration of the puzzle.

The next morning, when Alex found her puzzle had been scrambled, she called you
to help her to reset her puzzle to its original configuration (shown below) as
quickly as possible, so her father won't realize that the puzzle was torn and
scrambled. Of course, you should do it using only valid movements, as above
described.

```
1 2 3
4 5 6
7 8 9
```

Your task is to write a program that, given a configuration, finds a way to set
the puzzle to its original configuration spending the minimum possible number
of moves to accomplish it, if the given puzzle is solvable. If this is not the
case, the program should point it out.


## Input

The problem input consists of several cases, each one defined by three lines
that describe a puzzle configuration. That is, lines correspond to a top-down
description of the rows of the given configuration, and each line consist of
three digits, separated by one blank character. The end of the input is
indicated by a line with a number '0'.

### Sample Input

```
2 3 1
4 5 6
7 8 9
7 3 9
2 5 1
4 8 6
1 2 3
4 5 6
7 9 8
0
```


## Output

For each puzzle in the input, you must print a line containing S, the minimum
number of moves required to set the puzzle to its original configuration,
followed by a space and 2 × S characters indicating any sequence of S moves
that solves the puzzle. A move is described by two characters: the first one
must be 'H' or 'V' (H specifies a horizontal move, and V a vertical move), and
the second one must be '1', '2', or '3' to indicate the row or the column to
move. If the puzzle is not solvable, you must output a line with the text 'Not
solvable'.

### Sample Output

```
1 H1
3 V1V3H1
Not solvable
```
