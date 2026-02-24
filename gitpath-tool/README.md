# gitpath-tool

A small Rust demo that shows how to:

* Read a value from a CLI flag
* Fall back to an environment variable
* Fall back to a default
* Run an external command

The git path is resolved in this order:

1. `--git-path`
2. `GIT` environment variable
3. `git` on your PATH

`--version` runs `git --version` using the resolved path.
`--print-git` prints the resolved path.

This project exists purely as a minimal example of CLI argument parsing and environment variable precedence using Rust and a single dependency (`clap`).

Build:

```
cargo build
```

Run:

```
cargo run -- --version
```