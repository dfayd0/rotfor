# RotFor

A small command-line tool for encoding and decoding ROT ciphers.

## Installation

```bash
git clone https://github.com/dfayd0/rotfor
cd rotfor
cargo install --path .
```

## Usage

```bash
rotfor encode --text "Hello" --shift 13
rotfor decode --text "Uryyb" --shift 13
rotfor decode --text "Uryyb" --guess
```

## Features

- Encode text with a custom shift.  
- Decode text using a known shift or automatically guess.

## Contributing

Feel free to open issues or pull requests.