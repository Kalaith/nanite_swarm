# Nanite Swarm - Tech Stack

> **Document Location:** `.project/tech-stack.md`
>
> This document outlines the technology choices and rationale for the project.
> All technology decisions should be documented here with reasoning.

---

## Stack Overview

```
┌─────────────────────────────────────────────────┐
│                   Frontend                       │
│  Macroquad + macroquad-toolkit + Immediate UI   │
├─────────────────────────────────────────────────┤
│                    Backend                       │
│  Rust Runtime + Macroquad + Local Logic         │
├─────────────────────────────────────────────────┤
│                   Data Layer                     │
│  JSON Files + Serde + Local Storage             │
├─────────────────────────────────────────────────┤
│                Infrastructure                    │
│  Static Web Hosting + Cargo Build + Manual      │
└─────────────────────────────────────────────────┘
```

---

## Core Technologies

### Language & Runtime

| Technology | Version | Purpose |
|------------|---------|---------|
| Rust | 2021 Edition | Primary language for game logic, memory safety, and WASM compilation |
| WebGL/WASM | ES6+ Browsers | Primary runtime for web deployment with hardware acceleration |
| Native Windows | Win32 API | Secondary runtime for desktop distribution |

**Rationale:**
- **Rust chosen over alternatives (JS/TS, C++, Python)**: Memory safety prevents crashes, zero-cost abstractions for performance, excellent WASM compilation support, strong type system catches errors at compile time
- **WebGL/WASM primary**: Enables browser-based gameplay without downloads, reaches widest audience, PRD requires <5s load time and 60 FPS - WASM delivers this
- **Native Windows secondary**: Provides desktop experience for users preferring native apps, leverages same codebase via conditional compilation
- **Alternatives considered**: JavaScript (too slow for 60 FPS grid simulations), C++ (unsafe, complex build), Python (interpreted, poor WASM support)

---

### Framework

| Technology | Version | Purpose |
|------------|---------|---------|
| Macroquad | 0.4.x | 2D game engine providing rendering, input, and window management |
| macroquad-toolkit | Latest | Immediate-mode UI framework for buttons, panels, and interactions |

**Rationale:**
- **Macroquad chosen over alternatives (SDL2, ggez, Amethyst)**: Immediate-mode rendering perfect for grid-based spatial puzzles, excellent WASM support, minimal abstraction overhead, active maintenance
- **macroquad-toolkit chosen over alternatives (imgui-rs, native Windows controls)**: Purpose-built for Macroquad, immediate-mode UI matches game development patterns, provides essential UI primitives without bloat
- **Framework selection driven by PRD requirements**: Grid-based rendering needs efficient 2D graphics, spatial puzzle mechanics require precise coordinate systems, research neural network needs dynamic drawing capabilities

---

### Database

| Technology | Version | Purpose |
|------------|---------|---------|
| JSON Files | N/A | Primary data store for game state and configuration |
| Serde | 1.0.x | Serialization/deserialization for save/load functionality |
| Local Storage | Browser API | Persistence layer for web deployment |

**Rationale:**
- **JSON chosen over alternatives (SQLite, IndexedDB)**: Simple file-based storage matches PRD requirement for export/import capabilities, human-readable for debugging, no database complexity needed for single-player game
- **Serde provides type-safe serialization**: Compile-time guarantees that save data matches code structures, handles versioning for future updates
- **Local Storage for web**: Simple key-value persistence for save files, works across browser sessions
- **Selection driven by PRD data requirements**: <50MB save files, indefinite retention, manual export/import - JSON perfectly fits this scope

---

## Dependencies

### Production Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| macroquad | 0.4 | Core 2D game engine - rendering, input, window management |
| macroquad-toolkit | Latest | UI framework - buttons, panels, input helpers |
| serde | 1.0 | Serialization framework with derive macros |
| serde_json | 1.0 | JSON serialization/deserialization |
| rand | 0.8 | Random number generation for procedural elements |

### Development Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| cargo-watch | Latest | File watching for automatic rebuilds during development |

---

## Build & Tooling

---

## Build & Tooling

### Build System

| Tool | Version | Purpose |
|------|---------|---------|
| Cargo | Built-in | Rust package manager and build system |
| wasm-pack | Latest | WASM packaging and optimization |

### Development Tools

| Tool | Purpose |
|------|---------|
| rustfmt | Code formatting and style consistency |
| clippy | Linting and code quality checks |
| cargo-watch | Automatic rebuilds during development |

### Build Commands

```bash
# Development (native)
cargo build

# Production build (native)
cargo build --release

# WebGL/WASM development
cargo build --target wasm32-unknown-unknown

# WebGL/WASM production
cargo build --release --target wasm32-unknown-unknown

# Code quality checks
cargo fmt && cargo clippy

# Development with auto-rebuild
cargo watch -x build
```

### Build Commands

```bash
# Development
[dev command]

# Production build
[build command]

# Testing
[test command]

# Linting
[lint command]
```

---

## Architecture Patterns

---

## Architecture Patterns

### Code Organization

```
nanite_swarm/
├── src/
│   ├── main.rs             # Entry point, game loop, phase transitions
│   ├── data/               # Data types and JSON loading
│   │   ├── mod.rs          # Re-exports all data types
│   │   ├── loader.rs       # JSON deserialization and constants
│   │   └── game_config.rs  # Game balance values and settings
│   ├── engine/             # Game logic services (stateless)
│   │   ├── mod.rs          # Re-exports
│   │   ├── grid_engine.rs  # Spatial calculations and pathfinding
│   │   ├── drone_engine.rs # Resource flow and automation logic
│   │   └── research_engine.rs # Neural network and tech progression
│   ├── state/              # State management
│   │   ├── mod.rs
│   │   ├── game_state.rs   # Current planetary state
│   │   └── persistence.rs  # Save/load functionality
│   ├── ui/                 # UI components
│   │   ├── mod.rs          # Re-exports toolkit functions
│   │   ├── core.rs         # Color schemes and styling
│   │   └── components.rs   # Game-specific UI widgets
│   └── screens/            # Screen renderers
│       ├── planetary_view.rs # Main grid gameplay
│       ├── research_view.rs  # Neural network interface
│       └── interplanetary_view.rs # Solar system overview
├── assets/
│   ├── game_config.json    # Balance values and constants
│   ├── research_tree.json  # Tech progression data
│   └── planet_configs/     # Per-planet settings
└── .project/               # Documentation
```

### Design Patterns Used

| Pattern | Where Used | Purpose |
|---------|------------|---------|
| State Machine | Game phases and planetary progression | Clear state transitions, no global mutable state |
| Immediate Mode UI | All user interfaces | Simple, stateless UI rendering |
| Data-Driven Design | Game balance and configuration | Easy iteration without recompilation |
| Entity Component | Grid tiles and buildings | Flexible object composition |
| Service Layer | Engine modules | Stateless business logic separation |

**Architecture Rationale:**
- **Modular separation** ensures UI never mutates state, engine services remain pure functions
- **Data-driven design** supports PRD requirement for JSON-based configuration
- **Immediate mode UI** matches Macroquad's rendering model and enables dynamic interfaces
- **State machine pattern** provides clear game phase management for planetary progression

---

## Environment Configuration

---

## Environment Configuration

### Required Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| None | Game runs entirely locally | No |

### Configuration Files

| File | Purpose |
|------|---------|
| `assets/game_config.json` | Game balance values, grid sizes, resource rates |
| `assets/research_tree.json` | Technology definitions and prerequisites |
| `assets/planet_configs/` | Per-planet hazard and resource settings |
| `Cargo.toml` | Rust dependencies and build configuration |

**Configuration Strategy:**
- **Build-time embedding**: All JSON assets embedded in WASM binary for instant loading
- **No runtime configuration**: Game settings immutable after compilation
- **Development overrides**: Environment variables can override asset paths for development

---

## External Services

---

## External Services

### APIs & Integrations

| Service | Purpose | Documentation |
|---------|---------|---------------|
| None | Fully self-contained game | N/A |

### Third-Party Services

| Service | Purpose | Account Required |
|---------|---------|------------------|
| Web Hosting | Static file hosting for WASM deployment | No |
| itch.io/GitHub Pages | Game distribution platforms | Optional |

**Service Independence:**
- **Zero external dependencies**: Game functions entirely offline after initial load
- **No API keys or accounts required**: Eliminates deployment complexity and user friction
- **Static hosting only**: Any web server can host the game files

---

## Security Considerations

---

## Security Considerations

### Authentication
- **None required**: Single-player local game with no user accounts

### Data Protection
- **Local storage only**: Save files stored in browser local storage or user filesystem
- **No sensitive data**: Game contains no personal information or credentials
- **Export capability**: Users can backup and transfer save files manually

### Dependencies
- **Regular updates**: Dependencies kept current to address security patches
- **Minimal surface area**: Only essential crates included
- **Code review**: All dependencies audited for security issues

**Security Approach:**
- **Rust memory safety**: Prevents buffer overflows and use-after-free vulnerabilities
- **No network communication**: Eliminates man-in-the-middle and server compromise risks
- **Open source transparency**: Code publicly auditable for security review

---

---

## Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| Frame Rate | 60 FPS | In-game performance monitor |
| Load Time | < 5 seconds | Time from page load to playable |
| Memory Usage (Web) | < 200MB | Browser dev tools memory tab |
| Memory Usage (Native) | < 100MB | OS task manager |
| Save File Size | < 50MB | File system size check |
| Build Time | < 30 seconds | Cargo build timing |

**Performance Strategy:**
- **Rust zero-cost abstractions**: High-level code compiles to efficient machine code
- **WASM optimization**: Release builds optimized for web deployment
- **Immediate mode rendering**: Minimal state management overhead
- **Data-driven design**: Fast JSON loading with Serde
- **Grid-based optimization**: Spatial data structures for efficient lookups

---

---

## Decision Log

| Date | Decision | Rationale | Alternatives Considered |
|------|----------|-----------|------------------------|
| 2026-01-24 | Rust + Macroquad for core engine | Memory safety, WASM compilation, performance for 60 FPS grid simulations | JavaScript (too slow), C++ (unsafe), Python (poor WASM) |
| 2026-01-24 | JSON for data persistence | Simple, human-readable, matches PRD export/import requirements | SQLite (overkill for local game), IndexedDB (browser-only) |
| 2026-01-24 | Immediate-mode UI | Perfect for dynamic interfaces, matches Macroquad rendering model | Retained-mode UI (state management overhead), HTML/CSS (not suitable for games) |
| 2026-01-24 | Data-driven configuration | Easy balancing without recompilation, supports iteration | Hardcoded constants (brittle), Lua scripting (complexity) |
| 2026-01-24 | Modular architecture | Clear separation of concerns, testable components | Monolithic structure (hard to maintain), ECS (over-engineering) |

---

*Last updated: 2026-01-24*
