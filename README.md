# minigrep

A simple command-line tool to search for a string within a file, inspired by the classic `grep` utility.

## Usage

To run the program with Cargo, use the following format. Note the `--` to separate the program arguments from Cargo's arguments.

```sh
cargo run -- <query> <filepath> [case_flag]
```

After building the project (`cargo build --release`), you can run the binary directly:

```sh
./target/release/minigrep <query> <filepath> [case_flag]
```

### Arguments

- `<query>`: The string you want to find.
- `<filepath>`: The path to the file you want to search in.
- `[case_flag]` (Optional): Determines if the search is case-insensitive/smartcase/sensitive.
  - `0`: case-insensitive search.
  - `1`: search with smartcase (means: **If your search pattern contains any uppercase letter, smartcase forces the search to be case-sensitive otherwise case-insensitive**).
  - `2` or omitted: Defaults to a case-sensitive search.

## Configuration via Environment Variables

You can also control case sensitivity by setting the `CASE` environment variable. This is useful for setting a default behavior for your entire shell session.

**Example (on Linux/macOS):**

```sh
# For a case-insensitive search by default
export CASE=0

# For searching with smartcase
export CASE=1

# For a case-sensitive search (by default)
export CASE=2
```

**Note:** A `case_flag` argument provided on the command line will always override the `CASE` environment variable for that specific command.
