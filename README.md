# Rust WebAssembly Project

This project is a simple Rust application compiled to WebAssembly (Wasm) that can be run in a web browser. It includes a static HTML page that interacts with the Rust code.

## Project Structure

```
rust-wasm-project
├── src
│   └── lib.rs          # Rust code compiled to WebAssembly
├── static
│   ├── index.html      # Main HTML page
│   └── style.css       # Styles for the HTML page
├── Cargo.toml          # Rust project configuration
├── README.md           # Project documentation
└── .gitignore          # Git ignore file
```

## Getting Started

### Prerequisites

- Rust and Cargo installed on your machine.
- `wasm-pack` installed. You can install it using the following command:

```bash
cargo install wasm-pack
```

### Building the Project

To build the project and generate the WebAssembly package, run:

```bash
wasm-pack build --target web
```

### Running the Project

You can serve the static files using a simple HTTP server. One option is to use `basic-http-server`, which can be installed via Cargo:

```bash
cargo install basic-http-server
```

Then, navigate to the `static` directory and run:

```bash
basic-http-server .
```

Open your web browser and go to `http://localhost:4000` to see the application in action.

## Contributing

Feel free to submit issues or pull requests if you have suggestions or improvements for the project.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.