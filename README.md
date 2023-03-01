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
The initial idea is that we have two sequences. First defined as F(n) = F(n-1) + F(n-2) + F(n-4) that means in n-th click F(n) copies where created. Second sequence as sum of all viruses in the world defined as V(n) = V(n-1) + F(n). The main task is to return V(n) for given n in the most efficient manner.
### Approach 1
The first and the most basic approach can be computing all values to the given n which gives us O(n).
### Approach 2
Second approach which could lead us to better time complexity so faster than O(n).

The mentioned sequence (1, 2, 3, 5 ...) adjusted to 0, 1, 2, 3, 5 ... while still holds the rules F(n) = F(n-1) + F(n-2) + F(n-4) looks very similar to classic Fibonacci sequence.  


