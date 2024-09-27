# `hex-ray`

A hexdump utility that let's you look at the individual bytes that make up a file. The value of each 8-bit byte is displayed as a pair of hexadecimal (base-16) values.

![demo](./docs/demo.gif)

---

## 📦 Installation

To install `hex-ray`, ensure you have Rust and Cargo installed. You can then build the project from source.

- Clone the repository.

    ```sh
    git clone https://github.com/Shresht7/hex-ray.git
    cd hex-ray
    ```

- Build the project and use the compiled binary located in the `target/release` directory.

    ```sh
    cargo build --release
    ```

    **Alternatively**, install directly using cargo

    ```sh
    cargo install --path .
    ```

---

## 📘 Usage

To use `hex-ray`, run the executable followed by the path of the file you want to inspect.

```sh
hex-ray <subcommand> [options]
```

or pipe something in

```sh
git log | hex-ray <subcommand>
```

### Commands

- `view`: View the hex-dump table
- `inspect`: View the hex-dump table in an interactive terminal UI
- `output`: Output only the values

    >[!TIP]
    > 
    > Use the `--help` flag for more information.

### Examples

- #### `cat ./src/main.rs | hex-ray view`

    ```output
    Source: STDIN
    ┌───────────┬─────────────────────────────────────────────────────┬─────────────────────┐
    │  ·····000 │ 2f 2f 20 54  72 61 69 74  73 0d 0a 75  73 65 20 63  │ //·T rait s··u se·c │
    │  ·····020 │ 6c 61 70 3a  3a 50 61 72  73 65 72 3b  0d 0a 0d 0a  │ lap: :Par ser; ···· │
    │  ·····040 │ 2f 2f 20 4d  6f 64 75 6c  65 73 0d 0a  6d 6f 64 20  │ //·M odul es·· mod· │
    │  ·····060 │ 63 6c 69 3b  0d 0a 6d 6f  64 20 75 74  69 6c 73 3b  │ cli; ··mo d·ut ils; │
    .........................................................................................
    │  ·····160 │ 4f 6b 28 72  65 74 29 0d  0a 7d 0d 0a               │ Ok(r et)· ·}··      │
    └───────────┴─────────────────────────────────────────────────────┴─────────────────────┘
    Read 636 bytes
    ```

- #### `cat ./src/main.rs | hex-ray view --plain`

    ```output
    00000000:  2f 2f 20 54  72 61 69 74  73 0d 0a 75  73 65 20 63   | //·T rait s··u se·c
    00000020:  6c 61 70 3a  3a 50 61 72  73 65 72 3b  0d 0a 0d 0a   | lap: :Par ser; ····
    00000040:  2f 2f 20 4d  6f 64 75 6c  65 73 0d 0a  6d 6f 64 20   | //·M odul es·· mod·
    00000060:  63 6c 69 3b  0d 0a 6d 6f  64 20 75 74  69 6c 73 3b   | cli; ··mo d·ut ils;
    ...
    ```

- #### `"Wow!" | hex-ray output --format binary`

    ```output
    01010111 01101111 01110111 00100001 00001101 00001010
    ```

## 📕 Additional Information


> [!NOTE]
>
> ## Why Hexadecimal?
>
> The number of distinct values any sequence of digits can represent is given by the base raised to the power of the number of digits in the sequence. For example, a 2-bit binary (base-2) sequence can represent 2<sup>2</sup> = 4 distinct values; namely 00, 01, 10, 11. The 8 binary digits in a byte can represent 2<sup>8</sup> = 256 distinct values, usually interpreted as the numbers 0 to 255. Similarly, two-digit hexadecimal numbers can represent 16<sup>2</sup> distinct values - also 256. This convenient correspondence is why programmers like hexadecimal notation so much - it gives us a compact way of representing all possible byte values with just two digits!
>
> Actually, it's even better than you might suspect at first. Each hexadecimal digit aligns cleanly with four bits of the corresponding byte so the hexadecimal number 0x12 corresponds to the byte 0001_0010 and the hexadecimal number 0x34 corresponds to 0011_0100. This makes it really easy to read bit patterns directly from hex notation!
>

> [!NOTE]
> 
> If you haven't met it before, the 0x prefix is used to indicate that a number is written in hexadecimal base. Similarly 0o can be used to indicate octal (base-8) and 0b to indicate binary (base-2).

### `NO_COLOR` Environment Variable

> [!TIP]
> `hex-ray` respects the `NO_COLOR` environment variable. ANSI colors will be disabled if `NO_COLOR` is set. You can pass in the `--no-color` flag to force disable the colors.

---

## License

This project is licensed under the [MIT License](./LICENSE). See the [LICENSE](./LICENSE) file for more details.
