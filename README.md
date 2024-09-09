# Minigrep

This is a toy command-line utility written in Rust for practice. It mimics unix utility called `grep`. Minigrep was created by following [Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) of the Rust Book.

## How to Run
To install, you need to have Rust language installed as well as `cargo` package manager. Clone this repository and run:

```shell
cargo run -- [string] [file-path]
```

In the command above `[string]` is the string you want to find and `[file-path]` is the path to the text file where you want to find the string.

Set `MINIGREP_IGNORE_CASE` environment variable if you want the search to be case-insensitive.

## Developer
Developed by zkkv, 08/2024
