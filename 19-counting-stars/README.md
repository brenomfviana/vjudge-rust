# Counting Stars

## Description

People generally don't care to give attention to stars in a moonlit night. In
most cases the attention goes towards the moon. Sadly, you have to write a
program now that can count the stars in the sky. For this problem a sky is a
two dimensional grid. Empty pixel is denoted by a '.' (ASCII value 46) and
a non-empty pixel is denoted by a '*' (ASCII value 42). As a star is a very
small object so it cannot occupy more than one pixel and in our sky two stars
are never adjacent. So two or more adjacent nonempty pixels can denote some
larger objects like moon, comet, sun or UFOs but they never represent a star.
All the eight possible pixels around a pixel are adjacent to it. In the figure
below the black pixel at the center have eight adjacent pixels. Of them three
pixels are non-empty.

```
*..
.**
..*
```


## Input

The input file contains at most 1000 sets of inputs. The description of each
set is given below:
Each set starts with two integer number r and c (0 < r, c < 101), which
indicates the row and column number of the image to follow. Next r rows
describe the sky as mentioned in the problem statement. Input is terminated by
a line containing two zeroes.

### Sample Input

```
5 5
.....
....*
....*
...*.
*....
4 3
...
.*.
...
*.*
0 0
```


## Output

For each set of input produce one line of output. This line contains a decimal
integer which denotes the number of stars in the given sky.

### Sample Output

``
1
3
``
