# Scarecrow

## Description

Taso owns a very long field. He plans to grow different types of crops in the
upcoming growing season. The area, however, is full of crows and Taso fears
that they might feed on most of the crops. For this reason, he has decided to
place some scarecrows at different locations of the field.

The field can be modeled as a
![equation](http://latex.codecogs.com/gif.latex?1%20%5Ctimes%20N) grid.
Some parts of the field are infertile and that means you cannot grow any crops
on them. A scarecrow, when placed on a spot, covers the cell to its immediate
left and right along with the cell it is on.

Given the description of the field, what is the minimum number of scarecrows
that needs to be placed so that all the useful section of the field is covered?
Useful section refers to cells where crops can be grown.


## Input

Input starts with an integer ![equation](http://latex.codecogs.com/gif.latex?t)
where ![equation](http://latex.codecogs.com/gif.latex?%28t%20%3C%3D%20100%29),
denoting the number of test cases. Each case starts with a line containing an
integer ![equation](http://latex.codecogs.com/gif.latex?n) where
![equation](http://latex.codecogs.com/gif.latex?%280%20%3C%20n%20%3C%20100%29).
The next line contains ![equation](http://latex.codecogs.com/gif.latex?n)
characters that describe the field. A dot (`.`) indicates a crop-growing spot
and a hash (`#`) indicates an infertile region.

### Sample Input

```
3
3
.#.
11
...##....##
2
##
```


## Output

For each case, output the case number first followed by the number of
scarecrows that need to be placed.

### Sample Output

```
Case 1: 1
Case 2: 3
Case 3: 0
```
