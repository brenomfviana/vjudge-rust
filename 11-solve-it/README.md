# Solve it

## Description

Solve the equation:

p ∗ e^{−x} + q ∗ sin(x) + r ∗ cos(x) + s ∗ tan(x) + t ∗ x^{2} + u = 0

where 0 ≤ x ≤ 1.


## Input

Input consists of multiple test cases and terminated by an EOF. Each test case
consists of 6 integers in a single line: p, q, r, s, t and u (where 0 ≤ p,
r ≤ 20 and −20 ≤ q, s, t ≤ 0). There will be maximum 2100 lines in the input
file.

### Sample Input

```
0 0 0 0 -2 1
1 0 0 0 -1 2
1 -1 1 -1 -1 1
```


## Output

For each set of input, there should be a line containing the value of x,
correct up to 4 decimal places, or the string ‘No solution’, whichever is
applicable.

### Sample Output

```
0.7071
No solution
0.7554
```
