# PGN Parser

This project can convert PGN files to CSV or JSON files.

## Usage

Writing to a csv file:
```bash
cargo run <input_file>.pgn <output_file>.csv csv
```

Writing to a json file:
```bash
cargo run <input_file>.pgn <output_file>.json json
```

## Note

This is still version 0.1. This release is stable, however it requires that 2 whitespaces are at the end of the file.
