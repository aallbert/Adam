# Rust Chessbot

A simple chess engine built in Rust

## Features

- **Chess Engine Core:**
  - **Move Generation:** Generates all legal moves for any given board position.
  - **Minimax Search:** Implements the minimax algorithm for optimal move selection.
  - **Perft Testing:** Includes perft tests for move generation validation at various depths.
- **Chess Logic:**
  - **FEN Support:** Parses and generates Forsyth-Edwards Notation (FEN) strings to represent board states.
  - **Move Representation:** Custom `ChessMove` struct for easy manipulation of moves.
  - **Piece-Square Tables:** Incorporates piece-square tables for improved static evaluation.
- **User Interface (Planned/Basic):**
  - Basic command-line interface for interacting with the engine.
  - Simple GUI for manual testing

## Directory Structure

```
.
└── src
├── core
│ ├── bestmv.rs #
│ ├── minimax.rs
│ ├── mod.rs
│ ├── movegen.rs # Move generation logic
│ └── movemasks.rs # Bitmasks for efficient move generation
├── models
│ ├── board.rs # ChessBoard struct and associated methods
│ ├── chessmove.rs # ChessMove struct
│ ├── mod.rs # Models module declarations
│ ├── piece.rs # Piece enum and related logic
│ └── piecesquaretables.rs # Piece-Square Tables for evaluation
├── gui.rs
├── interface.rs # helper functions
├── main.rs # Main application entry point
└── testing.rs # Unit and integration tests (e.g., perft tests)
```

## Getting Started

### Prerequisites

- Rust programming language: [https://www.rust-lang.org/](https://www.rust-lang.org/)

### Building and Running

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/aallbert/Adam.git
    cd Adam
    ```

2.  **Build the project:**

    ```bash
    cargo build --release
    ```

3.  **Run the application:**
    ```bash
    cargo run --release
    ```

**Alternatively** any GUI using the UCI Protcol can be used. En-croissant is a simple GUI used during development.

### Running Tests

To run the extensive test suite, including perft tests:

```bash
cargo test
```
