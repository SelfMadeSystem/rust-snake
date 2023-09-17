# Snake Rust

## Description

This is a simple snake game written in Rust using the [speedy2d](https://crates.io/crates/speedy2d) library.

## Build

To build the game, you need to have Rust installed. Then, you can run the following command:

```bash
cargo build --release
```

The executable will be located in the `target/release` folder.

## Run

If you already built the game, you can run it using the executable in the `target/release` folder.

If not, you can run the game using the following command:

```bash
cargo run --release
```

## Controls

- `W` or `Up Arrow` to move up
- `A` or `Left Arrow` to move left
- `S` or `Down Arrow` to move down
- `D` or `Right Arrow` to move right

## License

This project is licensed under the DBAD license. See the [LICENSE](LICENSE.md) file for more details.

## Credits

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [speedy2d](https://crates.io/crates/speedy2d)
- [rand](https://crates.io/crates/rand)
- [SelfMadeSystem](https://github.com/SelfMadeSystem/) (me)
- [icewormy3](https://github.com/icewormy3)'s teacher for telling her class to make
    a snake game in Java using LinkedLists, which inspired me try to make the game in
    Python using deque in under 1 hour. I then decided to make the game in Rust because
    I felt like it. The Python game is available in the [snek.py](snek.py) file.

