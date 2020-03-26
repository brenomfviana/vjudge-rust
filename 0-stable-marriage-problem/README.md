# Stable Marriage Problem

## Description

There are given ![equation](http://latex.codecogs.com/gif.latex?n) men and
![equation](http://latex.codecogs.com/gif.latex?n) women. Each woman ranks all
men in order of her preference (her first choice, her second choice, and so on).
Similarly, each man sorts all women according to his preference. The goal is to
arrange ![equation](http://latex.codecogs.com/gif.latex?n) marriages in such a
way that if a man ![equation](http://latex.codecogs.com/gif.latex?m) prefers
some woman ![equation](http://latex.codecogs.com/gif.latex?w) more than his
wife, and ![equation](http://latex.codecogs.com/gif.latex?w) prefers
![equation](http://latex.codecogs.com/gif.latex?m) more then her husband a new
marriage occurs between ![equation](http://latex.codecogs.com/gif.latex?w) and
![equation](http://latex.codecogs.com/gif.latex?m). If
![equation](http://latex.codecogs.com/gif.latex?w) prefers her husband more,
then she stays married to him. This problem always has a solution and your task
is to find one.

### Example

Two sets with four elements each.

Set ![equation](http://latex.codecogs.com/gif.latex?A) preferences:

```
1: 3 2 4 1
2: 2 3 1 4
3: 3 1 2 4
4: 3 2 4 1
```

Set ![equation](http://latex.codecogs.com/gif.latex?B) preferences:

```
1: 4 3 1 2
2: 2 1 3 4
3: 1 3 4 2
4: 4 3 1 2
```

The respective stable mathing set ![equation](http://latex.codecogs.com/gif.latex?%28a%2C%20b%29):

```
(1, 3)
(2, 2)
(3, 1)
(4, 4)
```


## Gale-Shapley's Pseudo Algorithm

```
Initialize each person (men and women) to be free.
while there is a man who is free and hasn't proposed to every woman:
  m: choose a free man
  w: the first woman on women's list to whom m has not yet proposed
  if w is free:
    m and w become engaged
  else // w is curently engaged to another man: m'
    if w prefers m to her fiancé m':
      m and w become engaged
      m' becomes free
    else
      w rejects m // m remains free!
Return the set S of engaged pairs (stable matching)
```


## Input

The first line contains a positive integer
![equation](http://latex.codecogs.com/gif.latex?t%3C%3D100) indicating the
number of test cases. Each test case is an instance of the stable marriage
problem defined above. The first line of each test case is a positive integer
![equation](http://latex.codecogs.com/gif.latex?n%3C%3D500) (the number
of marriages to find). The next
![equation](http://latex.codecogs.com/gif.latex?n) lines are the woman's
preferences: ith line contains the number
![equation](http://latex.codecogs.com/gif.latex?i) (which means that this is
the list given by the ![equation](http://latex.codecogs.com/gif.latex?i)th woman)
and the numbers of men (the first choice of
![equation](http://latex.codecogs.com/gif.latex?i)th woman, the second choice,
...). Then, the men's preferences follow in the same format.

### Sample Input

```
2
4
1 4 3 1 2
2 2 1 3 4
3 1 3 4 2
4 4 3 1 2
1 3 2 4 1
2 2 3 1 4
3 3 1 2 4
4 3 2 4 1
7
1 3 4 2 1 6 7 5
2 6 4 2 3 5 1 7
3 6 3 5 7 2 4 1
4 1 6 3 2 4 7 5
5 1 6 5 3 4 7 2
6 1 7 3 4 5 6 2
7 5 6 2 4 3 7 1
1 4 5 3 7 2 6 1
2 5 6 4 7 3 2 1
3 1 6 5 4 3 7 2
4 3 5 6 7 2 4 1
5 1 7 6 4 3 5 2
6 6 3 7 5 2 4 1
7 1 7 4 2 6 5 3
```


## Output

For each test case print ![equation](http://latex.codecogs.com/gif.latex?n)
lines, where each line contains two numbers
![equation](http://latex.codecogs.com/gif.latex?m) and
![equation](http://latex.codecogs.com/gif.latex?w), which means that
the man number ![equation](http://latex.codecogs.com/gif.latex?m) and the
woman number ![equation](http://latex.codecogs.com/gif.latex?w) should get
married.

### Sample Output

```
1 3
2 2
3 1
4 4
1 4
2 5
3 1
4 3
5 7
6 6
7 2
```
