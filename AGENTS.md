# AGENTS.md

This document guides AI coding agents for the **Nanite Swarm** project. It is derived from `.project/prd.md`, `.project/tech-stack.md`, and `CODE_STANDARDS.md`.

This project uses the shared RustGames agent instructions in [`../AGENTS.md`](../AGENTS.md). Codex should read and apply that file when working here.

---

## Project Summary

Nanite Swarm is a 2D grid-based automation and logistics game where a self?replicating AI consumes planetary resources. The MVP focuses on **Planet 1** with automation, spatial logistics puzzles, terrain tradeoffs, and research. Interplanetary progression is post?MVP.

Core loop: **build ? automate ? research ? expand**

Pressure systems (MVP): dust accumulation, power collapse cascade, and irreversible terrain consequences.

---

## Scope & Priorities

### MVP (Planet 1)
- Grid-based resource automation with drones
- Non?overlapping conduits + bridges for water/void crossings
- Power grid with repeaters and power constraints
- Research tree with prerequisites and data generation
- Terrain utilization tradeoffs (harvest vs preserve) with permanent consequences
- Offline progression (battery + hibernation)

### Post?MVP
- Interplanetary persistence + resource transfer
- Planet-specific hazards (acid rain, freeze, etc.)

---

## Tech Stack (Do Not Deviate)

- **Language:** Rust 2021
- **Engine:** Macroquad
- **UI:** macroquad?toolkit (immediate mode)
- **Data:** JSON + Serde
- **Targets:** WebGL/WASM + Native Windows

---

## Architecture Rules

- **main.rs** owns the game loop and state transitions
- **data/** defines JSON?driven configuration (no engine/UI dependencies)
- **engine/** is stateless logic and calculations
- **state/** owns mutable game state
- **screens/** render UI and return actions
- **UI never mutates state directly**

---

## Data?Driven Design (Hard Requirement)

All balance values and constants must live in JSON under `assets/` and be loaded at startup. Do not introduce hardcoded tuning constants in Rust unless the JSON wiring is added.

---

## Coding Standards (Key Enforcement)

- No unused variables or fields (remove them; do not suppress)
- Avoid variable shadowing
- Prefer small functions (20?50 lines, max 100)
- Use `Option`/`Result` for fallible work (avoid panics)
- Comments explain **why**, not **what**

---

## Current Implemented Systems

- Grid + fog of war
- Buildings: Core, Drill, Conduit, Bridge, Power Node, Wind Turbine, Server Bank, Sweeper, Storage, Biomass Harvester
- Dust accumulation + countermeasures
- Power collapse cascade (soft failure)
- Forest filters + permanent terrain scars
- Research tree (including unlock?gated buildings)
- Short?term directives with rewards
- Core micro?stage visuals
- Procedural art generator

---

## Development Workflow

### Run
```bash
cargo run
```

### Generate Art
```bash
cargo run --bin build_graphics
```

### Web Build
```bash
cargo build --release --target wasm32-unknown-unknown
```

---

## Agent Behavior Guidelines

- Respect the PRD scope: **do not** expand to post?MVP unless requested
- Follow CODE_STANDARDS.md conventions
- Keep all tuning numbers in `assets/game_config.json`
- If adding a new system, update JSON config + data structs
- Preserve existing UI style; no UI rewrites without explicit request

---

## Key Files

- `.project/prd.md`
- `.project/tech-stack.md`
- `CODE_STANDARDS.md`
- `assets/game_config.json`
- `src/main.rs`
- `src/engine/`
- `src/state/`
- `src/screens/`

---

*Last updated: 2026-01-26*
