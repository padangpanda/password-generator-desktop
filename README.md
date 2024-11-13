# Password Generator

A Rust application for generating a password that's using [Slint](https://slint.rs/) for the user interface and rust.

## About

This template helps you get started developing a Rust application with Slint as toolkit
for the user interface. It demonstrates the integration between the `.slint` UI markup and
Rust code, how to react to callbacks, get and set properties, and use basic widgets.

## Usage
1. Build with `cargo`:
    ```
    cargo build
    ```
2. Run the application binary:
    ```
    cargo run
    ```
    
    if you use WSL
    ```
    LIBGL_ALWAYS_SOFTWARE=1 cargo run
    ```

We recommend using an IDE for development, along with our [LSP-based IDE integration for `.slint` files](https://github.com/slint-ui/slint/blob/master/tools/lsp/README.md). You can also load this project directly in [Visual Studio Code](https://code.visualstudio.com) and install our [Slint extension](https://marketplace.visualstudio.com/items?itemName=Slint.slint).
