# Interactive Bill Manager (Rust)

A robust command line application built to manage daily expenses. This project was developed as a hands on exercise to master the fundamentals of Rust, specifically focusing on memory management, collections, and error handling.

## Features
- **Create & Update:** Add new bills and modify existing ones using a single interface.
- **Read:** View a formatted list of all active bills.
- **Delete:** Remove bills by name with a fast lookup.
- **Navigation:** Integrated "back" functionality to cancel operations mid way.
- **Safety:** Input validation ensures the program won't crash on invalid numeric entries.

## Tech Stack
- **Language:** Rust 
- **Data Structure:** `std::collections::HashMap`
- **Compiler Tools:** `cargo check`, `cargo fmt`, `rust-analyzer`

## Rust Concepts Implemented
- **Ownership & Borrowing:** Efficiently passing strings and references between functions.
- **Pattern Matching:** Using `match` to handle user selections and `Result`/`Option` types.
- **Mutability (`mut`):** Managing state changes in the `HashMap` and input buffers.
- **String Sanitization:** Handling terminal newline characters (`\n`) using `.trim()`.

## Getting Started

### Prerequisites
Ensure you have the Rust toolchain installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh