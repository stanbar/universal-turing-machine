# Universal Turing Machine

Unsigned integer summator implemented on universal turing machine.


## Theory

Turing machine is an abstract machine used as a conceptual mathematical model
of computation. Turing machine is the simplest model capable of executing any
algorithm. It uses band (a.k.a input/output), head (pointing at a cell on the
band) and it's own state 

Universal Turing Machine is a generalization of Turing machine which can
simulate any arbitrary Turing machine on arbitrary input. It achieve it by
geting as an input not only the band, but also a state-transition table.

## Usage

Execute `cargo run` and enter two numbers.

## Example

```
$ cargo run

--- Bits addition using Turing Machine ---

Enter two unsigned integers to sum:
12
3

Input tape is: 1100_11
0       [1] 1  0  0  _  1  1
0        1 [1] 0  0  _  1  1
0        1  1 [0] 0  _  1  1
0        1  1  0 [0] _  1  1
0        1  1  0  0 [_] 1  1
1        1  1  0  0  _ [1] 1
1        1  1  0  0  _  1 [1]
1        1  1  0  0  _  1  1 [_]
2        1  1  0  0  _  1 [1] _
3        1  1  0  0  _ [1] 0  _
3        1  1  0  0 [_] 1  0  _
4        1  1  0 [0] _  1  0  _
0        1  1  0  1 [_] 1  0  _
1        1  1  0  1  _ [1] 0  _
1        1  1  0  1  _  1 [0] _
1        1  1  0  1  _  1  0 [_]
2        1  1  0  1  _  1 [0] _
2        1  1  0  1  _ [1] 1  _
3        1  1  0  1 [_] 0  1  _
4        1  1  0 [1] _  0  1  _
4        1  1 [0] 0  _  0  1  _
0        1  1  1 [0] _  0  1  _
0        1  1  1  0 [_] 0  1  _
1        1  1  1  0  _ [0] 1  _
1        1  1  1  0  _  0 [1] _
1        1  1  1  0  _  0  1 [_]
2        1  1  1  0  _  0 [1] _
3        1  1  1  0  _ [0] 0  _
3        1  1  1  0 [_] 0  0  _
4        1  1  1 [0] _  0  0  _
0        1  1  1  1 [_] 0  0  _
1        1  1  1  1  _ [0] 0  _
1        1  1  1  1  _  0 [0] _
1        1  1  1  1  _  0  0 [_]
2        1  1  1  1  _  0 [0] _
2        1  1  1  1  _ [0] 1  _
2        1  1  1  1 [_] 1  1  _
5        1  1  1  1  _ [1] 1  _
5        1  1  1  1  _  _ [1] _
5        1  1  1  1  _  _  _ [_]
Steps: 40
Band lenght: 8
Output: 12 + 3 = 15 = 0b1100 + 0b11 = 0b1111
```


