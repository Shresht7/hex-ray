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

### Formats

All subcommands support the `--format` option, which dictates the format of the output values.

| Format                            | Example Output | Accepted Options                             |
| --------------------------------- | :------------: | -------------------------------------------- |
| Hexadecimal                       |      `3f`      | `"hex"`, `"x"`, `"hexadecimal"`              |
| Hexadecimal with Prefix           |     `0x3f`     | `"#hex"`, `"#x"`, `"#hexadecimal"`           |
| Uppercase Hexadecimal             |      `3F`      | `"HEX"`, `"X"`, `"Hex"`, `"Hexadecimal"`     |
| Uppercase Hexadecimal with Prefix |     `0x3F`     | `"#HEX"`, `"#X"`, `"#Hex"`, `"#Hexadecimal"` |
| Binary                            |   `00111111`   | `"binary"`, `"b"`, `"bin"`                   |
| Binary with Prefix                |  `0b00111111`  | `"#binary"`, `"#b"`, `"#bin"`                |
| Octal                             |     `077`      | `"octal"`, `"o"`, `"oct"`                    |
| Octal with Prefix                 |    `0o077`     | `"#oct"`, `"#o"`, `"#oct"`                   |
| Decimal                           |     `063`      | `"decimal"`, `"d"`, `"dec"`                  |

> [!TIP]
> The `--format` option also works for the interactive terminal UI.

#### Examples

- `hex-ray view ./src/main.rs --offset 500 --limit 50 --format binary --size 4`
    
    ```
    Source: ./src/main.rs
    ┌───────────┬──────────────────────────────────────┬──────┐
    │  ·····364 │ 00111101 00111110 00100000 01100011  │ =>·c │
    │  ·····370 │ 01101101 01100100 00101110 01100101  │ md.e │
    │  ·····374 │ 01111000 01100101 01100011 01110101  │ xecu │
    │  ·····000 │ 01110100 01100101 00101000 00101001  │ te() │
    │  ·····004 │ 00111111 00101100 00001101 00001010  │ ?,·· │
    │  ·····010 │ 00100000 00100000 00100000 00100000  │ ···· │
    │  ·····014 │ 00100000 00100000 00100000 00100000  │ ···· │
    │  ·····020 │ 01010011 01101111 01101101 01100101  │ Some │
    │  ·····024 │ 00101000 01100011 01101100 01101001  │ (cli │
    │  ·····030 │ 00111010 00111010 01000011 01101111  │ ::Co │
    │  ·····034 │ 01101101 01101101 01100001 01101110  │ mman │
    │  ·····040 │ 01100100 00111010 00111010 01001001  │ d::I │
    │  ·····044 │ 01101110 01110011                    │ ns   │
    └───────────┴──────────────────────────────────────┴──────┘
    Read 50 bytes
    ```

- The same in octal with prefix. `hex-ray view ./src/main.rs --offset 500 --limit 50 --format "#oct" --size 4`

    ```
    Source: ./src/main.rs
    ┌───────────┬──────────────────────────┬──────┐
    │  ·····364 │ 0o075 0o076 0o040 0o143  │ =>·c │
    │  ·····370 │ 0o155 0o144 0o056 0o145  │ md.e │
    │  ·····374 │ 0o170 0o145 0o143 0o165  │ xecu │
    │  ·····000 │ 0o164 0o145 0o050 0o051  │ te() │
    │  ·····004 │ 0o077 0o054 0o015 0o012  │ ?,·· │
    │  ·····010 │ 0o040 0o040 0o040 0o040  │ ···· │
    │  ·····014 │ 0o040 0o040 0o040 0o040  │ ···· │
    │  ·····020 │ 0o123 0o157 0o155 0o145  │ Some │
    │  ·····024 │ 0o050 0o143 0o154 0o151  │ (cli │
    │  ·····030 │ 0o072 0o072 0o103 0o157  │ ::Co │
    │  ·····034 │ 0o155 0o155 0o141 0o156  │ mman │
    │  ·····040 │ 0o144 0o072 0o072 0o111  │ d::I │
    │  ·····044 │ 0o156 0o163              │ ns   │
    └───────────┴──────────────────────────┴──────┘
    Read 50 bytes
    ```

---

## 📕 Additional Information


> [!NOTE]
>
> ## Why Hexadecimal?
>
> The number of distinct values any sequence of digits can represent is given by the base raised to the power of the number of digits in the sequence. For example, a 2-bit binary (base-2) sequence can represent 2<sup>2</sup> = 4 distinct values; namely 00, 01, 10, 11. The 8 binary digits in a byte can represent 2<sup>8</sup> = 256 distinct values, usually interpreted as the numbers 0 to 255. Similarly, two-digit hexadecimal (base-16) numbers can represent 16<sup>2</sup> distinct values - also 256. This convenient correspondence is why programmers like hexadecimal notation so much - it gives us a compact way of representing all possible byte values with just two digits!
>
> Actually, it's even better than you might suspect at first. Each hexadecimal digit aligns cleanly with four bits of the corresponding byte so the hexadecimal number 0x12 corresponds to the byte 0001_0010 and the hexadecimal number 0x34 corresponds to 0011_0100. This makes it really easy to read bit patterns directly from hex notation!
>

> [!NOTE]
> 
> The `0x` prefix indicates that a number is written in hexadecimal (base-16) format. Similarly, `0o` is used to indicate octal (base-8) and `0b` to indicate binary (base-2).

### `NO_COLOR` Environment Variable

> [!TIP]
> `hex-ray` respects the `NO_COLOR` environment variable. ANSI colors will be disabled if `NO_COLOR` is set. You can pass in the `--no-color` flag to force disable the colors.

---

## License

This project is licensed under the [MIT License](./LICENSE). See the [LICENSE](./LICENSE) file for more details.
