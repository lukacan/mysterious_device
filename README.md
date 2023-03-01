# The Mysterious Device

## Assignment
The mysterious device is reproducing a virus. After the first four presses, it
creates 1, 2, 3, 5 copies of the virus. With each subsequent click, the viruses
multiply, so that the viruses created by the last, second to
last, and fourth to last click create
one copy of themselves. Find out how many viruses are there in the world after
Andrej clicks the button X times.

### Input and Output
In the first line of the input is the number 1<=T<=1000 - it determines the
number of questions (number of following lines). In each of the next T lines
there is an integer 4<=X<=10^10. For each X, count how many viruses there are
in the world. Since there can be really many, just list its remainder after dividing
by 10^9 + 7.


## Solution
### Overview
The initial idea is that we have two sequences. First defined as F(n) = F(n-1) + F(n-2) + F(n-4) that means in n-th click F(n) copies where created. Second sequence as sum of all viruses in the world defined as S(n) = F(n) + F(n-1) + F(n-2) + F(n-3) ... + F(0). The main task is to return S(n) for given n in the most efficient manner.
### Approach 1
The first and the most basic approach is computing all values to the given n which gives us O(n). With storing all values. We can find max from the input values, compute S(n) where n is max and then all S(n') where n'<= n would be O(1).
### Approach 2
Second approach which could lead us to better time complexity so faster than O(n).
<br />
<br />
Let`s assume:
<br />
```
F(n-4) + F(n-2) = F(n) - F(n-1)<br />
F(n-5) + F(n-3) = F(n-1) - F(n-2)<br />
F(n-6) + F(n-4) = F(n-2) - F(n-3)<br />
.
.
.
F(0) + F(2) = F(4) - F(3)<br />
S(n-4) = F(n) - F(3) - (S(n-2) - F(0) - F(1))
```   
so
<br />

S(n-4) = F(n) - F(3) - (S(n-4) + F(n-3) + F(n-2) - F(0) - F(1))<br />
2(S(n-4)) = F(n) - F(3) - F(n-3) - F(n-2) + F(0) + F(1)

That leads us to the question if it`s possible to find F(n) in better than O(n). We have to find formula for computing straight F(n) or try to find algorithm to compute F(n) in maybe O(log n). I tried to use Python to solve Linear reccurent function with constant coeficients, but did not find any that could descibe sequence correctly. Second try was to find Donald E. Knuth's matrix identity (used for Fibonacci sequence) that can lead to O(log n) with some effective matrix multiplication.
[The Nth Fibonacci Number in O(log N)](https://kukuruku.co/hub/algorithms/the-nth-fibonacci-number-in-olog-n), also unsuccessful.

### Final solution
Based on results from previous approaches, my solution works in O(n) time complexity. Memory complexity is also O(n), but this can be improved. Currently my solution stores all values until n. Certainly, that is not necenecessary because we know all requested ns in advance.


## Run
Compile:
```
rustc mysterious_device.rs
```
Run:
```
./mysterious_device
```

## Appendix
I created simple shell script for generating test inputs.<br />
Usage:
```
./generate_random.sh T X_Max
```
T = number of questions<br />
X_Max = max question number (4-X_Max)<br />
Output file is generated as "input_T", where content of file has format generateed exactly as required input:
```
T
Q1
Q2
Q3
.
.
.
QT
```


