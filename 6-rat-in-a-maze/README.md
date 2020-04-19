# Rat in the maze

## Description

We all have might have played the game called Maze in our childhood. A maze is
a path or collection of paths, typically from an entrance to a goal. Here, a rat
placed at (0, 0) in a square matrix m[][] of order n and has to reach the
destination at (n-1, n-1). Your task is to display a sorted array of strings
denoting all the possible directions which the rat can take to reach the
destination at (n-1, n-1). The directions in which the rat can move are
'U' (up), 'D' (down), 'L' (left), 'R' (right).


## Input

The first line of input contains an integer T denoting the no of test cases.
Then T test cases follow. Each test case contains two lines.
The first line contains an integer n denoting the size of the square matrix.
The next line contains n*n space separated values of the matrix m where 0's
represents blocked paths and 1 represent valid paths.

### Constraints

- 1 ≤ T ≤ 10
- 2 ≤ n ≤ 10

### Sample Input

```
2
4
1 0 0 0 1 1 0 1 0 1 0 0 0 1 1 1
4
1 0 0 0 1 1 0 1 1 1 0 0 0 1 1 1
```


## Output

For each test case output will be space separated sorted strings denoting all
directions, which the rat could take to reach the destination.


### Sample Output

```
DRDDRR
DDRDRR DRDDRR
```

## Explanation

Consider the second testcase,

```
1 0 0 0
1 1 0 1
1 1 0 0
0 1 1 1
```

For the above matrix the rat can reach the destination at (3, 3) from (0, 0) by
two paths ie DRDDRR and DDRDRR when printed in sorted order we get DDRDRR
DRDDRR.
