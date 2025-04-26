[![Build Status (nightly)](https://github.com/sigurd4/poengsum/workflows/Build-nightly/badge.svg)](https://github.com/sigurd4/poengsum/actions/workflows/build-nightly.yml)
[![Build Status (nightly, all features)](https://github.com/sigurd4/poengsum/workflows/Build-nightly-all-features/badge.svg)](https://github.com/sigurd4/poengsum/actions/workflows/build-nightly-all-features.yml)

[![Build Status (stable)](https://github.com/sigurd4/poengsum/workflows/Build-stable/badge.svg)](https://github.com/sigurd4/poengsum/actions/workflows/build-stable.yml)
[![Build Status (stable, all features)](https://github.com/sigurd4/poengsum/workflows/Build-stable-all-features/badge.svg)](https://github.com/sigurd4/poengsum/actions/workflows/build-stable-all-features.yml)

[![Test Status](https://github.com/sigurd4/poengsum/workflows/Test/badge.svg)](https://github.com/sigurd4/poengsum/actions/workflows/test.yml)
[![Lint Status](https://github.com/sigurd4/poengsum/workflows/Lint/badge.svg)](https://github.com/sigurd4/poengsum/actions/workflows/lint.yml)

[![Latest Version](https://img.shields.io/crates/v/poengsum.svg)](https://crates.io/crates/poengsum)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Documentation](https://img.shields.io/docsrs/poengsum)](https://docs.rs/poengsum)
[![Coverage Status](https://img.shields.io/codecov/c/github/sigurd4/poengsum)](https://app.codecov.io/github/sigurd4/poengsum)

# poengsum
A very simple CLI tool that counts points and keeps track of teams competing in a quiz.

## Input

You write the score of each team in a file called `poengsum.txt`, like in this example:

```json
Nester Grønn: 13 20
Det klør på quizzen: 6 12
Brew wars: Return of the Heimert: 6 29 9
2. Etg.: 12 10
Camp Royal: 32 20 25
The Boratas: 22 20
De Tilfeldige: 20 14 9
Kate's Bush: 42.5 21 42.5
Blank: 21 16
Navigators (gir aldri opp): 19.5 17 9
The Good Seeds: 49 47 47
Sølvkre: 32 24 13
Dreit på draget laget: 23 21 16
Guranarane: 36.5 31 45
De anonyme: 11 13
Null peiling: 11 13
```

Each line contains one team, followed by a colon, and then whitespace separated point values for each round. Team names can include a colon. The points follow the last colon on each line. Teams that have not participated in all rounds will have a score of 0 in those rounds they were not participating in.

## Output

You can then display the leaderboard for the total sum of all rounds thus far by entering `poengsum` in the terminal, in the same directory as `poengsum.txt`.

`$ poengsum`
```
16. Det klør på quizzen: 18
15. 2. Etg.: 22
13. De anonyme: 24
13. Null peiling: 24
12. Nester Grønn: 33
11. Blank: 37 ↓3
10. The Boratas: 42 ↓3
9. De Tilfeldige: 43 ↑2
8. Brew wars: Return of the Heimert: 44 ↑2
7. Navigators (gir aldri opp): 45.5 ↑2
6. Dreit på draget laget: 60
5. Sølvkre: 69 ↓1
4. Camp Royal: 77 ↑1
3. Kate's Bush: 106
2. Guranarane: 112.5
1. The Good Seeds: 143
```

You can also see the amount of positions that each team climbed or fell since the previous round, denoted by the arrow after their score. If your terminal supports colored text, then these will also be colored red and green for a more pleasant viewing experience.

You can also specify a specific sequence of rounds as command-line arguments to the command `poengsum`, and it will display the score, as if these rounds were replayed in the given sequence.

For example, if i want to see how it would have played out if round 2 happened first, and then round 1 happened twice, i can do this:

`$ poengsum 2 1 1`
```
16. Det klør på quizzen: 24
15. 2. Etg.: 34
13. De anonyme: 35
13. Null peiling: 35
12. Brew wars: Return of the Heimert: 41 ↓2
11. Nester Grønn: 46 ↑1
10. De Tilfeldige: 54 ↑1
9. Navigators (gir aldri opp): 56
8. Blank: 58
7. The Boratas: 64
6. Dreit på draget laget: 67
5. Camp Royal: 84
4. Sølvkre: 88
3. Guranarane: 104 ↓1
2. Kate's Bush: 106 ↑1
1. The Good Seeds: 145
```

How convenient!