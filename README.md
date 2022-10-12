# automaton-runner

Unable to find any better name. Runs a deterministic finite automaton on an
input string, returning

- whether or not the automaton accepts the input and
- the trace of states the automaton went through

## Installation

1. Get yourself [Rust][rust-installation]

2. Clone the repository

  ```sh
  git clone https://github.com/MultisampledNight/automaton-runner
  ```

3. Navigate to the repo

  ```
  cd automaton-runner
  ```

4. Build and run the example!

  ```
  cargo run -- --automaton-path examples/email.txt --input doge@example.org
  ```

The source code is to be found in `src`, be aware that I really like to use
iterators. They're cool. Also the parser is buggy and likes overwriting things.

[rust-installation]: https://doc.rust-lang.org/stable/book/ch01-01-installation.html

## Writing an automaton

That's pretty simple! Every state is in its own "section". A section starts with
any line that doesn't contain a `=>`, and contains the state's name and its
"roles", separated by `#`. There's the following roles:

- `start` causes the state to be the entry point, there can be only one of those
- `end` signals to the runner "should the input run out of characters and end up
  here, it's accepted"
- `catch` is similar to `end`, but _rejects_ the input and specifically mentions
  the last state has been a catching one.

Then, in each line in a section you define what characters (or more to say,
unicode codepoints) would cause a transition (`=>`) to the next state.
Whitespace is completely ignored. A small example:

```
S0 # start
  a => S0
  b => S1
S1
  a => S0
  b => S2
S2 # end
  ab => S2
```

This automaton would only accept inputs where there are at least two `b`
following each other.
