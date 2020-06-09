# hare-and-hounds

## References

1. [wikipedia/Hare_games](https://en.wikipedia.org/wiki/Hare_games)
1. [nintendo/Club House Games](https://www.nintendo.com/games/detail/clubhouse-games-51-worldwide-classics-switch/#all-games)

## Usage

### Input Format

Text

```
.D...
D...R
.D...
```

(`3x5` characters)

is for

```
. D-.-. .
 /|\|/|\
D-.-.-.-R
 \|/|\|/
. D . . .
```

- `D` is a Hound <b>D</b>og.
    - Dogs running from left to right.
- `R` is a <b>R</b>abbit (Hare).
    - The Rabbit wanna escape to left.
- `.` is a just empty.

### Solver

```bash
cargo run --release -- --next D < input
cargo run --release -- --next R < input
```

See `--help` for more details!
