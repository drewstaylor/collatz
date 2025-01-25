# Collatz Conjecture
Solves the Collatz Conjecture for a given number range (start and end)

[Wikipedia Entry](https://en.wikipedia.org/wiki/Collatz_conjecture)

## Usage

```sh
# Invoke with start and end. 
./collatz 1 100

# Or
cargo run 1 100

# Defaults to start: 1, end: 100 if no start and end are declared at run time
```

See [code](https://github.com/drewstaylor/collatz/blob/b0a7bf99da5848ce25f513806aed506e1a761dcb/src/main.rs#L5-L6) for enabling and disabling debug output.
