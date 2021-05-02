# Universal Turing Machine

Unsigned integer summator implemented on universal turing machine.

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


