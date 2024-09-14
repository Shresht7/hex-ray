# `hex-ray`

A hexdump utility that let's you look at the individual bytes that make up a file. The value of each 8-bit byte is displayed as a pair of hexadecimal (base-16) values.

<!-- TODO: Add sample output, screenshot or demo -->

## Installation

To install `hex-ray`, ensure you have Rust and Cargo installed. You can then build the project from source.

1. Clone the repository.

```sh
git clone https://github.com/Shresht7/hex-ray.git
cd hex-ray
```

2. Build the project.

```sh
cargo build --release
```

3. The compiled binary will be located in the `target/release` directory.


## Usage

To use `hex-ray`, run the executable followed by the path of the file you want to inspect.

```sh
hex-ray <filepath>
```

### Sample Output

Here's an example of what the output will look like.

```output
00000000  48 65 6c 6c 6f 20 77 6f  72 6c 64 21 0a           | Hello world!.   |
```

<!-- TODO: Full help message -->

> [!TIP]
> `hex-ray` respects the `NO_COLOR` environment variable. ANSI colors will be disabled if `NO_COLOR` is set. You can pass in the `--no-color` flag to force disable the colors.

---

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

---

## License

This project is licensed under the [MIT License](./LICENSE). See the [LICENSE](./LICENSE) file for more details.
