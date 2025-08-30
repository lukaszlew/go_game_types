# Go Game Types

A Rust library providing core type definitions for the game of Go.

## Intent

This library is designed to facilitate easy and performant collaboration between various Go game libraries by providing a shared set of fundamental types. By using common type definitions, different Go libraries can interoperate seamlessly without conversion overhead.

## Types

This library exports the following types:

- **`Player`** - Represents a player in the game (Black or White)
- **`Color`** - Represents the color of a stone on the board
- **`Vertex`** - Represents a position on the Go board

To import the types you need:

```rust
use go_game_types::{Player, Color, Vertex};
```
