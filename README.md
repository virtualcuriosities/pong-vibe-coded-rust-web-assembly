# Pong (Web Assembly), Vibe-coded in Rust with Copilot

This project is a simple Pong game made in Rust / Web Assembly by someone who doesn't know Rust or Web Assembly using Copilot.

See https://www.virtualcuriosities.com/articles/4935/coding-with-ai-for-the-first-time for the blog post.

## Project Structure

```
rust-wasm-project
├── ,vscode
│   └── tasks.json      # VS Code tasks configuration.
├── src
│   ├── lib.rs          # Main Rust code.
│   └── ball.rs         # Classes.
├── pkg                 # Generated after building
├── target              # Generated after building
├── static
│   ├── index.html      # Main HTML page
│   ├── style.css       # Styles for the HTML page
│   └── pkg             # symlink to ../pkg
├── Cargo.toml          # Rust project configuration
├── README.md           # Project documentation
└── .gitignore          # Git ignore file
```

### Building and Running the Project

Use the "Build Rust WebAssembly" task to compile Rust to web assembly (to pkg/)

Use the "Serve Static Files" task to run a Python3 server on static/ for locahost:8080.

There should be a symlink on static/pkg -> pkg. If your operating system or git client doesn't support symlinks, copy the pkg folder to inside the static folder.

Access localhost:8080 on your web browser after building to play the pong.

## Contributing

This project was published for educational purposes only and will not be updated. You are free to fork and learn from it, however.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.