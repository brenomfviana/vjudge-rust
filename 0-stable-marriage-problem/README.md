# Stable Marriage Problem (SMP)


## Description

This project present a rust solution (based on Gale-Shapley's algorithm) for
[Stable Marriage Problem (SMP)](https://en.wikipedia.org/wiki/Stable_marriage_problem) developed as a work for
Algorithms Project discipline.
The SMP is a search problem whose purpose is to find a stable matching set. Each match is composed by two elements
![equation](http://latex.codecogs.com/gif.latex?%28a%2C%20b%29) of two equally sized sets
![equation](http://latex.codecogs.com/gif.latex?A) and ![equation](http://latex.codecogs.com/gif.latex?B) where
![equation](http://latex.codecogs.com/gif.latex?a%20%5Cin%20A) and
![equation](http://latex.codecogs.com/gif.latex?b%20%5Cin%20B). Each element of both sets has a list of preferences
(an ordered set) corresponding to the elements of the other set. For example, the element
![equation](http://latex.codecogs.com/gif.latex?a_1) has the preferences
![equation](http://latex.codecogs.com/gif.latex?P%28a_1%29%20%3D%20%5Bb_1%2C%20b_4%2C%20b_3%2C%20b_2%5D) and the
element ![equation](http://latex.codecogs.com/gif.latex?b_4) has the preferences
![equation](http://latex.codecogs.com/gif.latex?P%28b_4%29%20%3D%20%5Ba_4%2C%20a_1%2C%20a_2%2C%20a_3%5D).

According to [Wikipedia](https://en.wikipedia.org/wiki/Stable_marriage_problem), the SMP has been stated as follows:

> Given ![equation](http://latex.codecogs.com/gif.latex?n) men and ![equation](http://latex.codecogs.com/gif.latex?n)
> women, where each person has ranked all members of the opposite sex in order of preference, marry the men and women
> together such that there are no two people of opposite sex who would both rather have each other than their current
> partners. When there are no such pairs of people, the set of marriages is deemed stable.

### Example

Two sets with four elements each.

Set ![equation](http://latex.codecogs.com/gif.latex?A) preferences:

![equation](http://latex.codecogs.com/gif.latex?%5C%5C1%3A%203%202%204%201%5C%5C%202%3A%202%203%201%204%5C%5C%203%3A%203%201%202%204%5C%5C%204%3A%203%202%204%201)

Set ![equation](http://latex.codecogs.com/gif.latex?B) preferences:

![equation](http://latex.codecogs.com/gif.latex?%5C%5C1%3A%204%203%201%202%5C%5C%202%3A%202%201%203%204%5C%5C%203%3A%201%203%204%202%5C%5C%204%3A%204%203%201%202)

The respective stable mathing set ![equation](http://latex.codecogs.com/gif.latex?%28a%2C%20b%29):

![equation](http://latex.codecogs.com/gif.latex?%5C%5C%281%2C%203%29%5C%5C%20%282%2C%202%29%5C%5C%20%283%2C%201%29%5C%5C%20%284%2C%204%29)


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


## Usage

To run the solver use the following command:

```
cargo run -- <file>
```

### Input and Output

The first line represents the number of test cases. The test cases must respect the following structure:
the next line represents the size ![equation](http://latex.codecogs.com/gif.latex?%28n%29) of the sets; the next
![equation](http://latex.codecogs.com/gif.latex?2n) lines represents the preferences of the elements of the set
![equation](http://latex.codecogs.com/gif.latex?B) and the set ![equation](http://latex.codecogs.com/gif.latex?A)
where the first number represents the element the the follow numbers the list of preferences.

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

The expected output:

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


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
