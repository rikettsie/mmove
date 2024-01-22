# MMOVE

This is a simple Rust program that I wrote with the purpose of learning [tokio-rs](https://github.com/tokio-rs/tokio), an asynchronous runtime for concurrency.

Three concurrent tasks are spawn and are managed in the main thread:

- One task is spawned to move the mouse pointer in a round indefinitely;
- Another task is spawned to gently kill the previous task after a while through a cancellation token;
- Another task is spawned to take the mouse pointer to a target position and make a couple of clicks;

The three tasks are awaited for completion and then all restart in a loop.

## Compilation and usage

In your terminal:

```bash
cargo build --release
```

Find the program `mmove` compiled under the release target directory (`target/release/`), and launch it:

```bash
mmove -x 140 -y 395 --v-offset 27
```

where `x` and `y` are the first target point to click, and `v-offset` is the vertical offset for the second point to be clicked.

Input parameters are parsed using the [clap](https://crates.io/crates/clap) crate.
