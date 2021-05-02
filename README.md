# Universal Turing Machine

Unsigned integer summator implemented on universal turing machine.


## Theory

Turing machine is an abstract machine used as a conceptual mathematical model
of computation. Turing machine is the simplest model capable of executing any
algorithm. It uses band (a.k.a input/output), head (pointing at a cell on the
band) and it's own state 

Universal Turing Machine (UTM) is a generalization of Turing machine which can
simulate any arbitrary Turing machine on arbitrary input. It achieve it by
geting as an input not only the band, but also a state-transition table.

UTM can be defined six-tuple M = (𝖰, Σ, ι, \_, 𝖠, 𝛿), where

- _Q_ is a finite set of states––inner machine state
- _Σ_ is a finite set of symbols––the tape alphabet
- _ι_ is the initial state, where _ι_ is in machine state _Q_
- \_ is the blank symbol, where \_ is in alphabet _Σ_
- _𝖠_ is the set of final states (subset of all states _Q_)––whenever machine reach the state, it terminate.
- _𝛿_ is a state-transition relation, Fromaly it's defined as a function `𝛿: (Q\𝖠 x Σ) → (Q x Σ x {L, S, R})`. The domain `(Q\𝖠 x Σ)` defines all machines states (minus the final states) and symbols on tape, the codomain `(Q x Σ x {L, R, S})` defines the new state of the machine, symbol writen to tape, and move of the head: Left, Right, or Stay,


## Summator

This source code implements both Universal Turing Machine (UTM), and concrete
algorithm––unsigned integer binary summator. The state-transition 𝛿 is as
follows:

```
𝛿(0,0) = (0,0,R)
𝛿(0,1) = (0,1,R)
𝛿(0,\_) = (1,\_,R)

𝛿(0,0) = (1,0,R)
𝛿(0,1) = (1,1,R)
𝛿(0,\_) = (2,\_,L)

𝛿(2,0) = (2,1,R)
𝛿(2,1) = (3,0,R)
𝛿(2,\_) = (5,\_,L)

𝛿(3,0) = (3,0,L)
𝛿(3,1) = (3,1,L)
𝛿(3,\_) = (4,\_,L)

𝛿(4,0) = (0,1,R)
𝛿(4,1) = (4,0,L)
𝛿(4,\_) = (0,1,R)

𝛿(5,0) = (5,\_,R)
𝛿(5,1) = (5,\_,R)
𝛿(5,\_) = (halt,\_,S)
```

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


