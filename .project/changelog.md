# Nanite Swarm - Changelog

> **Document Location:** `.project/changelog.md`
>
> All notable changes to this project will be documented in this file.
> Format based on [Keep a Changelog](https://keepachangelog.com/).

---

## [Unreleased]

### Added
- Enforced non-overlapping conduit placement
- Bridge tiles for conduit crossings
- Invalid placement feedback for blocked placements
- Power transmission range with repeater nodes
- Conduit auto-routing with obstacle avoidance
- Research progress system with active topic and progress bar
- Sell button for buildings with partial refunds
- Place-building bounce animation with particle burst
- Conduit autotiling (connected pipe rendering)
- Power node glow bleed and powered building pulse
- Drone trail rendering for moving drones
- Terrain variation for empty tiles
- Full UI overhaul with new HUD, build palette cards, intel sidebar, and command bar

### Changed
- Redesigned main menu, research, interplanetary, and settings screen layouts

### Fixed
- (empty)

---

## [1.0.0] - 2026-01-24

### Added
- Planetary hazards system (dust, acid rain, cryo effects)
- Evolving Core visual stages in planetary view
- Particle effects for drone movement and resource delivery
- Animated UI elements and hover pulses across screens
- Planetary background visuals (starfield and glow)
- Refined UI palette and panel/button styling
- Battery system with 4-hour capacity and UI readout
- Hibernation mode that throttles simulation after battery depletion
- Offline progress simulation during load
- Offline progress banner on load
- Added unit test for offline progression timing
- Drag-to-place building placement
- In-game help overlay and expanded controls hints
- Settings menu with basic preferences
- Save/load UI and file persistence hook
- Achievements tracking panel with milestone unlocks
- Visible-range grid rendering for large maps
- BFS-based drone pathfinding with terrain avoidance
- Batched drone resource deliveries
- WASM size optimization via rustflags and optional wasm-opt
- Optional FPS display for performance validation
- Updated WebGL index page and deployment script
- Dry-run deployment test for Windows and WebGL packaging
- Final build packages created in dist/

### Fixed
- WASM build support by enabling getrandom JS backend

---

## [0.4.0] - 2026-01-24

### Added
- Power grid connectivity system with flood-fill algorithm
- Buildings require power connection to Core via conduits/power nodes
- Unpowered building indicators (dimmed appearance, "!" warning)
- Net power display in top bar (green positive, red negative)
- Terrain harvesting system (H key to harvest)
- Mountain harvesting yields 15 minerals, transforms to Rough terrain
- Forest harvesting yields 10 biomass, transforms to Empty terrain
- Harvest preview highlighting and preservation bonus tooltips
- Complete research tree with 11 technologies in neural network visualization
- ResearchNode, ResearchTree, ResearchState in engine/research_engine.rs
- Interactive research_view.rs with hoverable nodes and click-to-research
- Server bank data generation (1 data/sec when powered)
- Research unlocking with data cost deduction
- Enhanced interplanetary_view.rs with 5 planets (Mercury, Venus, Mars, Jupiter, Saturn)
- Planet info panels with difficulty ratings and descriptions
- Animated planetary orbits
- Mass driver colonization mechanic (requires research, costs 100 minerals)
- Planet travel between colonized worlds
- Colonization status tracking with visual indicators

### Changed
- Main Game struct now tracks research_tree, research_state, and colonized_planets
- Interplanetary view now requires has_mass_driver flag for colonization
- Research instantly unlocks when clicked (no progress bar)

---

## [0.3.0] - 2026-01-24

### Added
- Complete grid system with procedural terrain generation
- TerrainType enum: Empty, Mountain, Forest, Water, Rough, Void
- Building types: Core, Drill, Conduit, PowerNode, WindTurbine, ServerBank
- Building placement with cost validation and hotkeys (1-5)
- Drone automation system with state machine (Idle, MovingToCore, MovingToDrill, Delivering, Error)
- DroneManager for spawning and updating drones
- Drill production timers with automatic drone dispatch
- Pathfinding (simple direct path algorithm)
- Fog of war / tile reveal system
- Mouse hover highlighting and placement preview
- Building toolbar with cost display
- Resource panel with drone count
- Wind turbine efficiency bonus on mountains (+100%)

### Changed
- PlanetState now contains Grid, DroneManager, and drill timers
- planetary_view.rs completely rewritten with full grid rendering
- Game loop now updates simulation each frame

---

## [0.2.0] - 2026-01-24

### Added
- Complete project scaffolding with Rust/Macroquad
- `src/` directory structure: data/, engine/, state/, ui/, screens/
- Cargo.toml with macroquad 0.4, serde, serde_json, rand dependencies
- Main game loop with window configuration (1280x720)
- GamePhase state machine (MainMenu, Playing, Research, Interplanetary)
- Data layer: GameConfig, GridConfig, ResourceConfig, BuildingConfig structs
- Engine layer: GridPos, Drone, ResearchNode foundational types
- State management: PlanetState, Resources, save/load functions
- UI foundation: Colors palette (PRD compliant), Dimensions, button/panel components
- Four game screens: main_menu, planetary_view, research_view, interplanetary_view
- assets/game_config.json with initial balance values
- Keyboard navigation: ESC (menu), R (research), M (map)

### Changed
- Added project to workspace in H:\RustGames\Cargo.toml

---

## [0.1.0] - 2026-01-24

### Added
- Initial project setup
- `.project/` documentation structure
- Product Requirements Document (prd.md)
- Tech Stack documentation (tech-stack.md)
- Build Plan with task tracking (build-plan.md)
- This changelog
- Game Design Document (gdd.md)

---

## Version Guidelines

### Version Format: `MAJOR.MINOR.PATCH`

- **MAJOR**: Breaking changes or significant milestones
- **MINOR**: New features, completed phases
- **PATCH**: Bug fixes, small improvements

### Change Types

| Type | Description |
|------|-------------|
| **Added** | New features or capabilities |
| **Changed** | Changes to existing functionality |
| **Deprecated** | Features marked for removal |
| **Removed** | Features that were removed |
| **Fixed** | Bug fixes |
| **Security** | Security-related changes |

---

## Milestones

| Version | Milestone | Date |
|---------|-----------|------|
| 1.0.0 | Production Release | TBD |
| 0.5.0 | Feature Complete (Phase 5) | TBD |
| 0.4.0 | Advanced Systems (Phase 4) | 2026-01-24 |
| 0.3.0 | Grid & Automation (Phase 3) | 2026-01-24 |
| 0.2.0 | Core Infrastructure (Phase 2) | 2026-01-24 |
| 0.1.0 | Project Setup (Phase 1) | 2026-01-24 |

---

*Last updated: 2026-01-24*
