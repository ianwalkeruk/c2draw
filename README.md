# C2Draw

A cross-platform GUI application for creating C4 model diagrams (C1 System Context and C2 Container diagrams). C2Draw provides an intuitive visual editor with export capabilities to C4-PlantUML and Mermaid formats.

![C2Draw Screenshot](screenshot.png)

## Features

- **Visual Diagram Editor**: Drag-and-drop interface for creating diagrams
- **C4 Model Support**: Full support for C1 (System Context) and C2 (Container) diagrams
- **Element Types**:
  - Person (internal and external)
  - Software Systems (internal and external)
  - Containers (Web Application, Database, Message Queue, Mobile App, Microservice)
- **Export Formats**:
  - C4-PlantUML (`.puml`)
  - Mermaid (`.mmd`)
  - Native JSON format (`.c4d`)
- **Cross-Platform**: Runs on Windows, macOS, and Linux

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) (1.70 or later)

### Build from Source

```bash
git clone https://github.com/yourusername/c2draw.git
cd c2draw
cargo build --release
```

The executable will be located at `target/release/c2draw`.

### Download Pre-built Binaries

Download the latest release from the [releases page](https://github.com/yourusername/c2draw/releases).

## Usage

### Running the Application

```bash
cargo run
```

Or run the built executable directly:

```bash
./target/release/c2draw
```

### Creating Diagrams

1. **Launch C2Draw**
2. **Add Elements**: Click buttons in the left sidebar to add:
   - Person (👤)
   - External Person
   - Software System (🖥️)
   - External System
   - Web Application
   - Database (🗄️)
   - Message Queue (📨)
3. **Arrange Elements**: Drag elements on the canvas to position them
4. **Edit Properties**: Select an element and edit its name/description in the right panel
5. **Export**: Use the Export menu to generate C4-PlantUML or Mermaid code

### File Operations

- **New**: Create a new diagram (File → New)
- **Open**: Load an existing `.c4d` file (File → Open)
- **Save**: Save the current diagram (File → Save)
- **Save As**: Save with a new name (File → Save As)

### Exporting Diagrams

#### C4-PlantUML

1. Create your diagram
2. Go to **Export → C4-PlantUML...**
3. Copy the generated code
4. Paste into a PlantUML-compatible editor or renderer

#### Mermaid

1. Create your diagram
2. Go to **Export → Mermaid...**
3. Copy the generated code
4. Paste into a Mermaid-compatible editor (GitHub, Notion, etc.)

## Example

### System Context Diagram

```
┌──────────────┐      Uses      ┌──────────────────┐
│    👤 User   │ ──────────────> │  🖥️ My System    │
└──────────────┘                 └──────────────────┘
```

### Container Diagram

```
┌──────────────┐      Uses      ┌──────────────────┐
│    👤 User   │ ──────────────> │ 🌐 Web App       │
└──────────────┘                 │   (React)        │
                                 └────────┬─────────┘
                                          │ Queries
                                          v
                                 ┌──────────────────┐
                                 │ 🗄️ Database      │
                                 │   (PostgreSQL)   │
                                 └──────────────────┘
```

## Architecture

```
c2draw/
├── src/
│   ├── main.rs          # Application entry point
│   ├── app.rs           # Main application state and UI
│   ├── model/           # Data models
│   │   ├── mod.rs       # Common types and traits
│   │   ├── elements.rs  # Diagram elements
│   │   ├── diagram.rs   # Diagram container
│   │   └── relationship.rs
│   ├── ui/              # UI components
│   │   ├── mod.rs
│   │   └── canvas.rs    # Diagram canvas
│   └── export/          # Export formats
│       ├── mod.rs
│       ├── plantuml.rs
│       └── mermaid.rs
└── Cargo.toml
```

## Technology Stack

- **UI Framework**: [egui](https://github.com/emilk/egui) + [eframe](https://github.com/emilk/egui/tree/master/crates/eframe)
- **Serialization**: [serde](https://serde.rs/) + serde_json
- **File Dialogs**: [rfd](https://github.com/PolyMeilex/rfd)
- **Unique IDs**: [uuid](https://docs.rs/uuid/)

## Development

### Project Structure

The codebase is organized into modules:

- **`model/`**: Core data structures for C4 diagrams
  - `Element`: Visual elements (Person, System, Container)
  - `Relationship`: Connections between elements
  - `Diagram`: Complete diagram with all elements and relationships

- **`ui/`**: User interface components
  - `Canvas`: Main diagram editing area with drag-and-drop
  - Helper functions for rendering elements

- **`export/`**: Export format generators
  - `PlantUmlExporter`: C4-PlantUML format
  - `MermaidExporter`: Mermaid C4 format

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run with logging
RUST_LOG=debug cargo run
```

### Testing

```bash
cargo test
```

## Roadmap

- [x] Basic diagram editing
- [x] C4-PlantUML export
- [x] Mermaid export
- [x] Save/load diagrams
- [ ] Relationship creation UI
- [ ] Undo/redo support
- [ ] Zoom and pan
- [ ] Component diagrams (C3)
- [ ] Code diagrams (C4)
- [ ] Multiple diagram views
- [ ] Custom element styling

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [C4 Model](https://c4model.com/) by Simon Brown
- [C4-PlantUML](https://github.com/plantuml-stdlib/C4-PlantUML) standard library
- [egui](https://github.com/emilk/egui) immediate mode GUI library

## Support

If you encounter any issues or have questions:

1. Check the [issues page](https://github.com/yourusername/c2draw/issues)
2. Create a new issue with a detailed description
3. Include steps to reproduce the problem
