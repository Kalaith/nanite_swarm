# Nanite Swarm Build Plan

> **CRITICAL INSTRUCTIONS FOR ENGINEERS**
>
> ## Project Structure
> All project documentation lives in the `.project/` directory at the repository root:
> ```
> .project/
> ├── prd.md           # Product Requirements Document
> ├── tech-stack.md    # Technology choices and rationale
> ├── build-plan.md    # This file - task tracking
> └── changelog.md     # Version history and updates
> ```
>
> ## Build Discipline
> 1. **Keep this document up to date** - Mark tasks as completed immediately after finishing them
> 2. **Build after every task** - Run the build command after completing each task
> 3. **Zero tolerance for warnings/errors** - Fix any warnings or errors before moving to the next task
> 4. **Update changelog.md** - Log significant changes, fixes, and milestones
>
> ```bash
> # Build command (run after each task)
> cargo build --release --target wasm32-unknown-unknown
> ```
>
> If warnings or errors appear, fix them immediately. Do not proceed until the build is clean.

---

## Status Legend

| Icon | Status | Description |
|------|--------|-------------|
| ⬜ | Not Started | Task has not begun |
| 🔄 | In Progress | Currently being worked on |
| ✅ | Completed | Task finished |
| ⛔ | Blocked | Cannot proceed due to external dependency |
| ⚠️ | Has Blockers | Waiting on another task |
| 🔍 | In Review | Pending review/approval |
| 🚫 | Skipped | Intentionally not doing |
| ⏸️ | Deferred | Postponed to later phase/sprint |

---

---

## Project Progress Summary

```
Phase 1: Project Setup         [████████████████████] 100%  ✅
Phase 2: Core Infrastructure   [████████████████████] 100%  ✅
Phase 3: Grid & Automation     [████████████████████] 100%  ✅
Phase 4: Advanced Systems      [████████████████████] 100%  ✅
Phase 5: Polish & Deployment   [░░░░░░░░░░░░░░░░░░░░]   0%  ⬜
─────────────────────────────────────────────────────────
Overall Progress               [████████████████░░░░]  80%
```

| Phase | Tasks | Completed | Blocked | Deferred | Progress |
|-------|-------|-----------|---------|----------|----------|
| Phase 1: Project Setup | 12 | 12 | 0 | 0 | 100% |
| Phase 2: Core Infrastructure | 24 | 24 | 0 | 0 | 100% |
| Phase 3: Grid & Automation | 30 | 30 | 0 | 0 | 100% |
| Phase 4: Advanced Systems | 20 | 20 | 0 | 0 | 100% |
| Phase 5: Polish & Deployment | 12 | 0 | 0 | 0 | 0% |
| **Total** | **98** | **86** | **0** | **0** | **88%** |

---

## Phase 1: Project Setup

### 1.1 Repository & Environment

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 1.1.1 | Initialize repository with .gitignore |
| ✅ | 1.1.2 | Create `.project/` directory structure |
| ✅ | 1.1.3 | Set up prd.md with requirements |
| ✅ | 1.1.4 | Set up tech-stack.md with technology choices |
| ✅ | 1.1.5 | Initialize changelog.md |
| ✅ | 1.1.6 | **BUILD CHECK** - Verify environment setup |

### 1.2 Project Foundation

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 1.2.1 | Create project scaffolding |
| ✅ | 1.2.2 | Configure build tooling |
| ✅ | 1.2.3 | Set up linting and formatting |
| ✅ | 1.2.4 | Configure environment variables |
| ✅ | 1.2.5 | Create base configuration files |
| ✅ | 1.2.6 | **BUILD CHECK** - Verify clean build with no warnings |

---

## Phase 2: Core Infrastructure

### 2.1 Project Scaffolding

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 2.1.1 | Create src/ directory structure (data/, engine/, state/, ui/, screens/) |
| ✅ | 2.1.2 | Set up Cargo.toml with all dependencies (macroquad, serde, etc.) |
| ✅ | 2.1.3 | Create main.rs with window configuration and basic game loop |
| ✅ | 2.1.4 | Implement basic state machine (GamePhase enum and transitions) |
| ✅ | 2.1.5 | Set up assets/ directory with placeholder JSON files |
| ✅ | 2.1.6 | **BUILD CHECK** - Verify clean build with no warnings |

### 2.2 Data Layer Foundation

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 2.2.1 | Create data/mod.rs and basic data structures (GridPos, Resources, etc.) |
| ✅ | 2.2.2 | Implement JSON loader for game configuration |
| ✅ | 2.2.3 | Define core game constants (grid size, resource types, building configs) |
| ✅ | 2.2.4 | Create game_config.json with basic balance values |
| ✅ | 2.2.5 | Implement GameConfig with Default trait for fallback values |
| ✅ | 2.2.6 | **BUILD CHECK** - Verify JSON structures compile correctly |

### 2.3 State Management

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 2.3.1 | Create state/mod.rs and PlanetState struct |
| ✅ | 2.3.2 | Implement basic planetary state (grid dimensions, resources) |
| ✅ | 2.3.3 | Add save/load functionality with Serde |
| ✅ | 2.3.4 | Create persistence.rs for JSON file operations |
| ✅ | 2.3.5 | Implement state serialization/deserialization |
| ✅ | 2.3.6 | **BUILD CHECK** - Verify state structs serialize correctly |

### 2.4 UI Foundation

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 2.4.1 | Create ui/mod.rs with component exports |
| ✅ | 2.4.2 | Implement ui/core.rs with color palette and styling constants |
| ✅ | 2.4.3 | Create basic UI components (buttons, panels, resource display) |
| ✅ | 2.4.4 | Implement action enums for user intents (MenuAction, PlanetaryAction, etc.) |
| ✅ | 2.4.5 | Create main menu, planetary view, research view, interplanetary view screens |
| ✅ | 2.4.6 | **BUILD CHECK** - Verify UI renders without crashes |

---

## Phase 3: Grid & Automation

### 3.1 Grid System Foundation

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 3.1.1 | Create engine/grid_engine.rs with Grid, Tile, GridPos structs |
| ✅ | 3.1.2 | Implement grid coordinate system with bounds checking, distance, neighbors |
| ✅ | 3.1.3 | Add terrain tile types (Empty, Mountain, Forest, Water, Rough, Void) |
| ✅ | 3.1.4 | Create basic grid rendering with terrain colors |
| ✅ | 3.1.5 | Implement grid reveal mechanics around buildings |
| ✅ | 3.1.6 | **BUILD CHECK** - Verify grid renders correctly |

### 3.2 Building System

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 3.2.1 | Define building types (Core, Drill, Conduit, PowerNode, WindTurbine, ServerBank) |
| ✅ | 3.2.2 | Implement building placement on grid with validation |
| ✅ | 3.2.3 | Add building rendering with color-coded letters |
| ✅ | 3.2.4 | Create building selection toolbar with hotkeys (1-5) |
| ✅ | 3.2.5 | Implement building cost and resource deduction |
| ✅ | 3.2.6 | **BUILD CHECK** - Verify buildings can be placed and rendered |

### 3.3 Resource Automation

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 3.3.1 | Create engine/drone_engine.rs with Drone, DroneManager, DroneState |
| ✅ | 3.3.2 | Implement ResourceType enum and resource accumulation |
| ✅ | 3.3.3 | Add drone spawning from drills on placement |
| ✅ | 3.3.4 | Implement drone pathfinding to Core (simple direct path) |
| ✅ | 3.3.5 | Create resource delivery and display in UI panel |
| ✅ | 3.3.6 | **BUILD CHECK** - Verify drones carry resources automatically |

### 3.4 Basic Logistics

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 3.4.1 | Implement conduit placement as building type |
| ✅ | 3.4.2 | Add visual drone movement rendering |
| ✅ | 3.4.3 | Create drill production timers and drone dispatch |
| ✅ | 3.4.4 | Implement terrain buildability validation |
| ✅ | 3.4.5 | Add drone cargo indicator visualization |
| ✅ | 3.4.6 | **BUILD CHECK** - Verify basic automation loop works |

### 3.5 Planetary View Screen

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 3.5.1 | Update screens/planetary_view.rs with full grid rendering |
| ✅ | 3.5.2 | Implement mouse hover and click for building placement |
| ✅ | 3.5.3 | Add placement preview and hover highlighting |
| ✅ | 3.5.4 | Create resource panel and building toolbar |
| ✅ | 3.5.5 | Integrate game update loop with state simulation |
| ✅ | 3.5.6 | **BUILD CHECK** - Verify planetary view is playable |

---

## Phase 4: Advanced Systems

### 4.1 Logistics Puzzle System

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 4.1.1 | Implement power grid connectivity with flood-fill algorithm |
| ✅ | 4.1.2 | Add conduits and power nodes for power transmission |
| ✅ | 4.1.3 | Create power grid with Core as power source |
| ✅ | 4.1.4 | Implement unpowered building detection and visual feedback |
| ✅ | 4.1.5 | Add power balance display and net power calculation |
| ✅ | 4.1.6 | **BUILD CHECK** - Verify power grid mechanics work |

### 4.2 Terrain Utilization

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 4.2.1 | Implement terrain harvest system (H key) |
| ✅ | 4.2.2 | Add mountain harvesting for minerals (+15) |
| ✅ | 4.2.3 | Create forest harvesting for biomass (+10) |
| ✅ | 4.2.4 | Implement terrain transformation (Mountain→Rough, Forest→Empty) |
| ✅ | 4.2.5 | Add harvest preview and preservation bonus tooltips |
| ✅ | 4.2.6 | **BUILD CHECK** - Verify terrain choices affect gameplay |

### 4.3 Research System

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 4.3.1 | Create engine/research_engine.rs with ResearchNode, ResearchTree, ResearchState |
| ✅ | 4.3.2 | Implement neural network visualization with node connections |
| ✅ | 4.3.3 | Add 11-node research tree with prerequisites and costs |
| ✅ | 4.3.4 | Create research_view.rs screen with hover tooltips |
| ✅ | 4.3.5 | Implement server bank data generation (1 data/sec when powered) |
| ✅ | 4.3.6 | **BUILD CHECK** - Verify research progression works |

### 4.4 Interplanetary Features

| Status | Task | Description |
|--------|------|-------------|
| ✅ | 4.4.1 | Implement planetary progression with colonization tracking |
| ✅ | 4.4.2 | Add mass driver mechanics (requires research, costs 100 minerals) |
| ✅ | 4.4.3 | Enhance interplanetary_view.rs with clickable planets and info panels |
| ✅ | 4.4.4 | Implement planet travel for colonized worlds |
| ✅ | 4.4.5 | Add animated orbits and visual planet differentiation |
| ✅ | 4.4.6 | **BUILD CHECK** - Verify interplanetary progression |

### 4.5 Planetary Hazards (Deferred to Phase 5)

| Status | Task | Description |
|--------|------|-------------|
| ⏸️ | 4.5.1 | Implement Mars-like dust accumulation |
| ⏸️ | 4.5.2 | Add Venus acid rain pipe damage |
| ⏸️ | 4.5.3 | Create Saturn cryo drone slowdown |
| ⏸️ | 4.5.4 | Add hazard mitigation technologies |
| ⏸️ | 4.5.5 | Implement hazard visual effects |
| ⏸️ | 4.5.6 | **BUILD CHECK** - Verify hazards create meaningful challenges |

---

## Phase 5: Polish & Deployment

### 5.1 Visual Polish

| Status | Task | Description |
|--------|------|-------------|
| ⬜ | 5.1.1 | Implement evolving Core visual stages |
| ⬜ | 5.1.2 | Add particle effects for drones and resources |
| ⬜ | 5.1.3 | Create smooth animations and transitions |
| ⬜ | 5.1.4 | Add background planetary visuals |
| ⬜ | 5.1.5 | Implement final color palette and styling |
| ⬜ | 5.1.6 | **BUILD CHECK** - Verify visual polish is complete |

### 5.2 Offline Mechanics

| Status | Task | Description |
|--------|------|-------------|
| ⬜ | 5.2.1 | Implement 4-hour battery system |
| ⬜ | 5.2.2 | Add hibernation mode (10% speed after 4 hours) |
| ⬜ | 5.2.3 | Create offline progress calculation |
| ⬜ | 5.2.4 | Add offline progress display on load |
| ⬜ | 5.2.5 | Test offline progression accuracy |
| ⬜ | 5.2.6 | **BUILD CHECK** - Verify offline mechanics work |

### 5.3 UI/UX Refinement

| Status | Task | Description |
|--------|------|-------------|
| ⬜ | 5.3.1 | Implement smart drag building placement |
| ⬜ | 5.3.2 | Add comprehensive tooltips and help system |
| ⬜ | 5.3.3 | Create settings and preferences menu |
| ⬜ | 5.3.4 | Implement save/load UI in main menu |
| ⬜ | 5.3.5 | Add achievement and progress tracking |
| ⬜ | 5.3.6 | **BUILD CHECK** - Verify UI is intuitive and complete |

### 5.4 Performance Optimization

| Status | Task | Description |
|--------|------|-------------|
| ⬜ | 5.4.1 | Optimize grid rendering for large maps |
| ⬜ | 5.4.2 | Implement efficient drone pathfinding |
| ⬜ | 5.4.3 | Add resource flow batching |
| ⬜ | 5.4.4 | Optimize WASM build size |
| ⬜ | 5.4.5 | Test performance on target hardware |
| ⬜ | 5.4.6 | **BUILD CHECK** - Verify 60 FPS and <200MB memory |

### 5.5 Deployment & Launch

| Status | Task | Description |
|--------|------|-------------|
| ⬜ | 5.5.1 | Create publish.ps1 deployment script |
| ⬜ | 5.5.2 | Set up index.html for WebGL hosting |
| ⬜ | 5.5.3 | Test deployment on multiple platforms |
| ⬜ | 5.5.4 | Create final build and packaging |
| ⬜ | 5.5.5 | Update changelog with v1.0.0 release |
| ⬜ | 5.5.6 | **FINAL BUILD CHECK** - Verify production deployment works |

---

## Changelog Reference

See `.project/changelog.md` for detailed version history.

**Recent Updates:**
- **v0.4.0** - Phase 4 Advanced Systems complete - power grid, research tree, interplanetary travel
- **v0.3.0** - Phase 3 Grid & Automation complete - playable game with terrain, buildings, drones
- **v0.2.0** - Phase 2 Core Infrastructure complete - full project scaffolding with working game loop
- **v0.1.0** - Project documentation framework established

---

## Notes & Decisions

### Architecture Decisions
- **Immediate-mode UI**: Matches Macroquad's rendering model, enables dynamic research neural networks
- **Data-driven design**: All balance values in JSON for easy iteration without recompilation
- **Modular engine services**: Clear separation between grid logic, drone AI, and research systems
- **State machine pattern**: Clean game phase management for planetary progression

### Implementation Priorities
- **Core loop first**: Grid, buildings, automation before advanced features
- **Visual feedback**: Drone activity and resource flow visualization critical for engagement
- **Progressive complexity**: Start with basic placement, add logistics puzzles, then interplanetary features

### Technical Considerations
- **WASM performance**: Grid operations must be optimized for 60 FPS target
- **Memory constraints**: <200MB limit requires efficient data structures
- **Cross-platform compatibility**: Same codebase for web and native builds

---

*Last updated: 2026-01-24*
*Current Phase: Phase 4 - Advanced Systems (Complete)*
*Next Milestone: Phase 5 - Polish & Deployment*
