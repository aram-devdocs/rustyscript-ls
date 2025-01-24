# RustyScript-LS

**RustyScript-LS** is a state-of-the-art, multithreaded TypeScript language server engineered in Rust. Its primary objective is to deliver high-performance developer tooling, including advanced features like autocomplete, type inference, and syntax validation, while ensuring modularity for extensibility. This architecture lays the groundwork for future innovations, such as constructing a TypeScript compiler entirely in Rust.

## Features

- **Comprehensive LSP Integration**: Seamless compatibility with editors like VS Code, Neovim, and other LSP clients.
- **High-Performance Multithreading**: Built on Rust and Tokio, leveraging asynchronous concurrency for exceptional speed and scalability.
- **Robust TypeScript Support**: Implements essential TypeScript features, including:
  - Intelligent autocompletion
  - Comprehensive type checking
  - Syntax validation
  - Contextual hover information
  - Detailed diagnostics
- **Extensible Modular Design**: Encapsulates language server logic, parsing, and type-checking into distinct, reusable components.

## Vision for Expansion

- Eliminate reliance on Node.js by implementing a Rust-based TypeScript compiler.
- Expand support for advanced TypeScript constructs such as generics, mapped types, and conditional types.
- Enable efficient `.d.ts` generation alongside optimized JavaScript output.

---

## Getting Started

### Prerequisites

- **Rust**: Install Rust via [rust-lang.org](https://www.rust-lang.org/tools/install).
- **TypeScript (Optional)**: If utilizing the TypeScript server (`tsserver`), ensure it is globally installed:
  ```bash
  npm install -g typescript
  ```

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/aram-devdocs/rustyscript-ls.git
   cd rustyscript-ls
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Launch the language server:
   ```bash
   cargo run
   ```

---

## Usage

### Integration with VS Code

1. Configure the VS Code TypeScript server path by creating a `.vscode/settings.json` file:

   ```json
   {
     "typescript.tsserver.path": "/path/to/rustyscript-ls/target/release/rustyscript-ls"
   }
   ```

2. Open a TypeScript project to enable RustyScript-LS features, including autocompletion, hover information, and diagnostics.

## Project Structure

```
rustyscript-ls/
├── Cargo.toml              # Project metadata and dependencies
├── README.md               # Documentation
└── src/
    ├── main.rs             # Entry point for the language server
    ├── lsp_server.rs       # Core implementation of LSP logic
    ├── ts_bridge.rs        # Optional bridge to tsserver
    ├── providers/          # LSP feature modules
    │   ├── completion.rs   # Autocomplete implementation
    │   ├── diagnostics.rs  # Syntax and type diagnostics
    │   ├── hover.rs        # Hover feature implementation
    │   └── mod.rs          # Provider module registry
    ├── parser/             # Future modules for TypeScript parsing
    ├── type_checker/       # Placeholder for type-checking logic
    └── utils.rs            # Shared utility functions
```

---

## Contributing

Contributions are welcomed! Follow these steps to get involved:

1. Fork the repository.
2. Create a branch for your feature or bugfix.
3. Submit a pull request with a detailed description of your changes.

### Contribution Ideas

- Expand LSP capabilities, such as symbol renaming and go-to definition.
- Integrate advanced diagnostics using SWC or custom parsers.
- Implement caching mechanisms for incremental builds to improve performance.

---

## Roadmap

- [ ] **Version 0.1**: Core LSP functionality, including autocomplete, hover, and diagnostics.
- [ ] **Version 0.2**: Incremental parsing and type-checking capabilities.
- [ ] **Version 1.0**: Complete TypeScript compiler replacement with a modular Rust-based architecture.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for further details.

---

## Acknowledgments

- [Tower-LSP](https://github.com/ebkalderon/tower-lsp): A robust Rust framework for implementing LSP servers.
- [SWC](https://swc.rs/): A high-performance Rust-based compiler infrastructure.
- [TypeScript](https://www.typescriptlang.org/): The foundation and inspiration for this project.
