C2Draw is a cross-platform program for easy creation of diagrams from the C4 model. It is written in Rust and provides a simply user interface that allows creation of C1 and C2 diagrams with minimal clutter. Output is C4-PlantUML compliant diagrams suitable for import into Markdown or Mermaid.


## Features

- Cross-platform support (Windows, Linux, macOS)
- Simple user interface
- C1 and C2 diagram creation
- C4-PlantUML compliant output
- Markdown and Mermaid support

## Technology

The project is written in idiomatic Rust and uses the following libraries:
- [egui](https://github.com/emilk/egui) for the user interface
- [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) for the application framework
- [serde](https://github.com/serde-rs/serde) for serialization and deserialization
- [serde_json](https://github.com/serde-rs/json) for JSON support
- [c4_plantuml](https://github.com/plantuml-stdlib/C4-PlantUML) for C4-PlantUML support
- [mermaid](https://github.com/mermaid-js/mermaid) for Mermaid support
- [markdown](https://github.com/commonmark/commonmark-spec) for Markdown support

## Quality Assurance and Testing

The project uses the following tools/crates for quality assurance and testing:

- [clippy](https://github.com/rust-lang/rust-clippy) for linting
- [rustfmt](https://github.com/rust-lang/rustfmt) for code formatting
- [cargo-audit](https://github.com/RustSec/rustsec) for security auditing
- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin) for code coverage
- [cargo-test](https://doc.rust-lang.org/cargo/commands/cargo-test.html) for unit testing
- [cargo-doc](https://doc.rust-lang.org/cargo/commands/cargo-doc.html) for documentation generation
- [cargo-build](https://doc.rust-lang.org/cargo/commands/cargo-build.html) for building the project
- [cargo-run](https://doc.rust-lang.org/cargo/commands/cargo-run.html) for running the project
- [cargo-check](https://doc.rust-lang.org/cargo/commands/cargo-check.html) for checking the project
- [cargo-clean](https://doc.rust-lang.org/cargo/commands/cargo-clean.html) for cleaning the project
- [cargo-update](https://doc.rust-lang.org/cargo/commands/cargo-update.html) for updating dependencies
- [egui-kittest](https://github.com/emilk/egui/tree/master/crates/egui_kittest) for UI testing