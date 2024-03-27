# chaos-grid

This is a (somewhat) simple brute force analysis of the chaos-grid game from [Sapphire828](https://manifold.markets/EstMtz?r=YmVubWFubnM)'s game [Chaos Grid](https://manifold.markets/EstMtz/-the-chaos-grid-place-your-bets-see?r=YmVubWFubnM) on [Manifold](https://manifold.markets?referrer=benmanns). [Note: I get a small bonus if you sign up with my links]

## The Game

Welcome to The Chaos Grid! At market close, 36 dice will be rolled by the fairlyrandom bot and arranged in a 6x6 grid filling up rows left to right and top to bottom. 1 point will be assigned for each triplet in the grid either vertically or horizontally (diagonals Do Not count). In the cases of quadruplets, pentuplets, or sextuplets, multiple points will be assigned (see example scoring below). Side bets for quadruplets, pentuplets, or sextuplets are also available.

Scoring: Horizontal and vertical
- Triplet = 1
- Quadruplet ("Quad") = 2
- Pentuplet ("Pent") = 3
- Sextuplet ("Hex") = 4

Range of Scores [0–48]

### Example 1

|       |       |       |       |   |       |
|-------|-------|-------|-------|---|-------|
| **1** | **1** | **1** | 3     | 4 | 4     |
| 4     | 1     | 6     | 4     | 5 | 5     |
| 2     | 3     | 5     | 5     | 1 | 6     |
| **3** | **3** | **3** | **3** | 2 | **2** |
| 6     | 4     | 2     | 2     | 3 | **2** |
| 5     | 1     | 4     | 3     | 3 | **2** |

Example 1 would resolve: 4=YES, Any Quad=YES

### Example 2

|   |   |   |   |   |   |
|---|---|---|---|---|---|
| 1 | 3 | 1 | 3 | 4 | 4 |
| 4 | 1 | 6 | 4 | 5 | 5 |
| 2 | 3 | 5 | 5 | 1 | 6 |
| 3 | 3 | 1 | 3 | 2 | 2 |
| 6 | 4 | 2 | 2 | 3 | 6 |
| 5 | 1 | 4 | 3 | 3 | 2 |

Example 2 would resolve: 0=YES

### Example 3

|       |   |   |   |   |   |
|-------|---|---|---|---|---|
| 1     | 4 | 2 | 3 | 6 | 5 |
| **4** | 1 | 3 | 3 | 4 | 1 |
| **4** | 6 | 5 | 1 | 2 | 4 |
| **4** | 4 | 5 | 3 | 2 | 3 |
| **4** | 5 | 1 | 2 | 3 | 3 |
| **4** | 5 | 6 | 2 | 6 | 2 |

Example 3 would resolve: 3=YES, Any Pent=YES

### Example 4

|   |   |       |       |       |   |
|---|---|-------|-------|-------|---|
| 1 | 4 | 4     | 5     | **5** | 4 |
| 4 | 1 | 6     | 4     | **5** | 2 |
| 2 | 3 | **5** | **5** | **5** | 6 |
| 3 | 3 | 1     | 3     | **5** | 2 |
| 6 | 4 | 2     | 2     | **5** | 6 |
| 5 | 1 | 4     | 3     | **5** | 2 |

Example 4 would resolve: 5=YES, Any Hex=YES

What will be the total score of the inaugural Chaos Grid?

I will also award a 500 mana bounty for the first full probability distribution calculation. I thought about this problem for a while and couldn't get very far. What is the probability of each total score 0-48? Private message me your calculations or post them in the comments to help others.

Enjoy! 🎲🎲🎲🎲🎲🎲🎲🎲🎲🎲

## The Analysis

```shell
$ cargo build --release
$ time target/release/chaos-grid --iterations 10000000000                                                                                 ~/src/github.com/marketpredicts/chaos-grid
Score: 0 Count: 2957495307 (29.57%)
Score: 1 Count: 3278189757 (32.78%)
Score: 2 Count: 2125954918 (21.26%)
Score: 3 Count: 1029552537 (10.30%)
Score: 4 Count: 409613613 (4.10%)
Score: 5 Count: 140470889 (1.40%)
Score: 6 Count: 42812032 (0.43%)
Score: 7 Count: 11871981 (0.12%)
Score: 8 Count: 3059016 (0.03%)
Score: 9 Count: 746970 (0.01%)
Score: 10 Count: 177839 (0.00%)
Score: 11 Count: 41794 (0.00%)
Score: 12 Count: 10066 (0.00%)
Score: 13 Count: 2408 (0.00%)
Score: 14 Count: 639 (0.00%)
Score: 15 Count: 162 (0.00%)
Score: 16 Count: 50 (0.00%)
Score: 17 Count: 17 (0.00%)
Score: 18 Count: 3 (0.00%)
Score: 19 Count: 2 (0.00%)
Run: 3 Count: 5657646474 (56.57646474%)
Run: 4 Count: 1216418970 (12.1641897%)
Run: 5 Count: 153017425 (1.53017425%)
Run: 6 Count: 15421824 (0.15421824%)
target/release/chaos-grid --iterations 10000000000  12790.59s user 74.55s system 1110% cpu 19:18.43 total
```
