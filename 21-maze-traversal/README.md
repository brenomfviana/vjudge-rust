# Maze Traversal

## Description

A common problem in artificial intelligence is negotiation of a maze. A maze
has corridors and walls. The robot can proceed along corridors, but cannot go
through walls.


## Input

The input begins with a single positive integer on a line by itself indicating
the number of the cases following, each of them as described below. This line
is followed by a blank line, and there is also a blank line between two
consecutive inputs.

For this problem, you will input the dimensions of a maze, as two integer
values on the first line. Of the two numbers, the first is the number of rows
and the second is the number of columns. Neither the number of rows nor columns
will exceed 60.

Following these numbers will be a number of rows, as specified previously. In
each row there will be a character for each column, with the row terminated by
the end of line. Blank spaces are corridors, asterisks are walls. There needn't
be any exits from the maze.

Following the maze, will be an initial row and column specified as two integers
on one line. This gives the initial position of the robot. Initially the robot
will be facing North (toward the first row).

The remaining input will consist of commands to the robot, with any amount of
interspersed whitespace. The valid commands are:

R rotate the robot 90 degrees clockwise (to the right)
L rotate the robot 90 degrees counter-clockwise (to the left)
F move the robot forward, unless a wall prevents this, in which case do nothing
Q quit the program, printing out the current robot row, column and orientation

### Sample Input

```
1
7 8
********
* * * **
* *    *
* * ** *
* * *  *
*   * **
********
2 4
RRFLFF FFR
FF
RFFQ
```


## Output

For each test case, the output must follow the description below. The outputs
of two consecutive cases will be separated by a blank line.

The final row and column must be integers separated by a space. The orientation
must be one of N, W, S, E and separated from the column by a space.

### Sample Output

```
5 6 W
```
