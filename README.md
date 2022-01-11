# The Ergex Regular Expression Library

## Features
Ergex is a regular expression library with a few overarching design goals:

#### POSIX-Compatible Matching and Submatching
Ergex supports (almost) POSIX-compatible matching, including
POSIX-compatible submatch extraction.

#### UTF-8 and Byte-Oriented Matching
Ergex supports matching both UTF-8 encoded text and raw bytes, and the
two encodings may be mixed in the same expression.

#### Push-Oriented
Unlike most regular expression libraries, ergex is *push-oriented*.
That means that it looks for matches even when the input data is separated
widely in time or space.

The most common expected use case would be for network traffic analysis:
ergex can take the contents of network packets as they arrive.
However, ergex was originally written for a reimplementation of the
[sam](https://github.com/deadpixi/sam) text editor, which has a dual-process
architecture in which parts of the file may be widely separated across a
network connection.

Push-orientation is rare; the only major example I'm aware of is
[HyperScan](https://hyperscan.io). Being able to read from streams (in a
pull fashion) is more common but still relatively rare.

#### Simultaneous Matching
Ergex supports matching arbitrarily many expressions *simultaneously*.
That is, as you push data to ergex, it will test all of the expressions
you've provided at once and report back all that match.

Regular expressions are compiled into "databases" consisting of arbitrarily
many expressions.

#### No Allocations at Matching Time
Ergex performs no memory allocations during matching.
A "scratch" structure is allocated to store state during matching;
the scratch structure is of a fixed size and can be reused.

#### Thread-Safe, Lock-Free Matching
Multiple threads, each with their own scratch structures, can perform
matching independently.

#### No Pathological Cases
There are no pathological expressions: expressions are matched in
`O(n*m)` time, where `n` is the length of the expression and `m` is the
input.

#### Selective Disabling of Expressions
Ergex allows expressions in a given scratch space to be selectively disabled.
It uses a novel data structure (called a `ShrinkSet`) to allow resetting of the
enabled set in `O(1)` time.

#### Fast Enough
Ergex is fairly fast. Running `cargo test --release` takes about ten seconds on
my laptop.

## Credits
Ergex stands on the shoulders of giants: it uses the excellent `regex-syntax` crate
for parsing expressions.

## Current Status
Ergex is absolutely still a work in progress. Most of the goals stated above have been
met but the API is absolutely not where I want it to be and there is essentially no
documentation. There is a fairly hefty test suite, however.

Due to some stuff coming up in my personal life (everything's okay, thank you), I haven't
been able to devote as much time as I'd like to ergex lately -- I hope to get back to it
soon but I put everything here on GitHub in case anyone was interested in it.

## License
Please see the `LICENSE` file.

## Contact
Drop me a line at `rob@frigidriver.com`.