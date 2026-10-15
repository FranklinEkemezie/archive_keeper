# Archive Keeper

Archive Keeper is a terminal-based Rust application for managing a small document archive.

## Features

- Register documents with title, author, category, and year
- View all registered documents
- Search by title, author, category, or document ID
- Borrow and return documents
- View archive statistics

## Requirements

- Rust toolchain

## Run

```bash
cargo run
```

## Test

```bash
cargo test
```

## Notes

- The archive exists only for the lifetime of the process.
- Document identifiers are assigned automatically.
- The project uses the Rust standard library only.