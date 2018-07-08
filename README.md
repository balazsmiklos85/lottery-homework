# Lottery homework

## Author

Miklós BALÁZS (balazsmiklos85@gmail.com)

## Building

    cargo build --release

(As the project currently does not have any third party dependencies,
theoretically it should be buildable with just `rustc` as well.)

### Dependencies

* `cargo`
* `cc` linker

(The project was built with rust 1.27, but older versions should work fine as
well.)

### Installing dependencies

* The recommended way to install `cargo` (and `rustc`, and `rustfmt`, etc.) is
with using `rustup`:
    
        curl https://sh.rustup.rs -sSf | sh

* The `cc` linker should be packaged for the distribution
    * Debian
            sudo apt-get install build-essential
    * openSUSE
            sudo zypper install gcc

## Solution

The solution algorithm is quite simple: all games are read up into byte (`u8`)
arrays (`Vec`), problematic games are simply skipped. Then when an input comes
it is also converted into an array of bytes, the previously loaded games get
iterated over, to count how many numbers are matching with the current input
(referenced in the code as "drawing"). The matching numbers are also counted
with an iteration, this time with the iteration of the drawing array, calling
`contains()` on the game (which also iterates over the game).

### Asymtotic run time

With all these in mind, given the number of the games (`g`), and the length of
a lottery drawing (`l`), the asymtotic run time is

$O(g * l ^ 2)$

### Possible optimization

This is a quite naive approach. As the current solution worked well on my
computer and provided its results in less that 1 second, I left it as it is.
However, possibilities of future optimizations are pointed out in the code in
place as TODO comments. (Not only for speed optimizations, but also for code
organization.)

### Comments

I recognise that the task description asked for code comments for better
understanding, but I believe in self-documenting code.
