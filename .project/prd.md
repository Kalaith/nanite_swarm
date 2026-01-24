# Nanite Swarm - Product Requirements Document

> **Document Location:** `.project/prd.md`
>
> This document defines the product requirements, features, and specifications.
> Keep this document as the single source of truth for what we're building.

---

## Overview

### Problem Statement
Traditional idle games require constant manual clicking and lose progress when advancing to new content. Players want engaging automation games that respect their time while providing meaningful progression and strategic depth. The market lacks sophisticated logistics strategy games that combine idle mechanics with spatial puzzle-solving and interplanetary resource management.

### Solution
Nanite Swarm is a self-replicating AI simulation where players control an artificial intelligence that consumes planetary bodies. The MVP focuses on a complete, polished **Planet 1** experience with automation, logistics puzzles, terrain tradeoffs, and research. **Interplanetary progression is post‑MVP** and will be introduced after Planet 1 is stable and balanced.

### Target Users
- **Primary:** Strategy game enthusiasts aged 25-45 who enjoy automation, resource management, and puzzle-solving
- **Secondary:** Casual gamers looking for relaxing idle experiences with strategic depth, mobile players seeking hybrid PC/mobile gameplay

### Success Metrics
- [ ] Achieve 10,000+ downloads in first 6 months
- [ ] Maintain 70%+ player retention rate across first 3 planets
- [ ] Average session length of 20+ minutes
- [ ] Positive user reviews focusing on automation and strategic depth

---

## Features

### Core Features (MVP)

#### Feature 1: Planetary Resource Automation
**Priority:** P0 (Must Have)

**Description:**
Players deploy a Core AI on a planet that automatically extracts resources through drone networks, eliminating manual clicking while providing strategic resource management.

**User Story:**
> As a player who values their time, I want the game to handle basic resource gathering automatically so that I can focus on strategic decision-making rather than repetitive clicking.

**Acceptance Criteria:**
- [ ] Rovers automatically spawn from drills and carry resources to the Core
- [ ] No manual clicking required for basic resource collection
- [ ] Visual feedback shows drone activity and resource flow
- [ ] Resources accumulate automatically during offline periods

**Technical Notes:**
- Implement drone pathfinding and resource carrying animations
- Support offline progression simulation

---

#### Feature 2: Logistics Puzzle System
**Priority:** P0 (Must Have)

**Description:**
Resource pipes cannot overlap, creating spatial puzzles where players must strategically plan conduit placement to avoid crossing resource lines.

**User Story:**
> As a strategy player, I want complex spatial challenges so that resource routing becomes a meaningful puzzle rather than trivial placement.

**Acceptance Criteria:**
- [ ] Pipes cannot overlap on the same grid cells
- [ ] Bridge tiles required for crossing different resource types
- [ ] Visual indicators show pipe conflicts
- [ ] Power grid requires repeater nodes for long distances

**Technical Notes:**
- Grid-based collision detection for pipes
- Visual feedback for invalid placements

---

#### Feature 3: Interplanetary Progression
**Priority:** P1 (Post‑MVP)

**Description:**
Players maintain progress when moving between planets, using resources from conquered worlds to fuel colonization of more difficult planets.

**User Story:**
> As a completionist player, I want to retain my progress when advancing so that my investment in early planets provides long-term benefits.

**Acceptance Criteria:**
- [ ] All progress persists between planets
- [ ] Mass drivers enable resource export between worlds
- [ ] Early planet resources can be used on later planets
- [ ] Planetary bonuses stack across the solar system

**Technical Notes:**
- Persistent save system across planet transitions
- Resource transfer mechanics between planets

---

#### Feature 4: Terrain Utilization Choices
**Priority:** P0 (Must Have)

**Description:**
Each terrain tile offers meaningful choices between harvesting for immediate resources or preserving for ongoing benefits like energy efficiency.

**User Story:**
> As a strategic player, I want meaningful terrain decisions so that my choices about resource utilization create replayability and tactical depth.

**Acceptance Criteria:**
- [ ] Mountains: Strip-mine for iron bonus vs preserve for wind turbine efficiency
- [ ] Forests: Consume for biomass vs preserve as pollution buffer
- [ ] Terrain choices affect long-term gameplay balance
- [ ] Visual feedback shows terrain transformation effects

**Technical Notes:**
- Tile state management with multiple utilization options
- Long-term effect calculations

---

#### Feature 5: Research and Evolution System
**Priority:** P0 (Must Have)

**Description:**
Visual neural network research tree where unlocking nodes expands the AI's intelligence and unlocks new capabilities.

**User Story:**
> As a player interested in progression, I want a visual research system so that my AI's growth feels tangible and rewarding.

**Acceptance Criteria:**
- [ ] Glowing neural network visualization
- [ ] Research requires data from server banks
- [ ] Server banks consume power and generate heat
- [ ] Unlocked nodes expand the neural network visually

**Technical Notes:**
- Visual node connection system
- Research prerequisite chains

---

### Secondary Features (Post-MVP)

#### Feature 6: Multi-Planet Hazards
**Priority:** P1 (Should Have)

**Description:**
Each planet introduces unique environmental challenges like acid rain, extreme temperatures, and dust storms that require specific technological solutions.

**User Story:**
> As an exploration enthusiast, I want varied planetary challenges so that each world feels distinct and requires different strategies.

---

#### Feature 7: Offline Progression Mechanics
**Priority:** P1 (Should Have)

**Description:**
AI continues running at reduced speed during player absence, with battery life management encouraging regular check-ins.

**User Story:**
> As a busy player, I want meaningful offline progress so that I can enjoy the game in short sessions without losing momentum.

---

#### Feature 8: Visual Core Evolution
**Priority:** P2 (Nice to Have)

---

### MVP Scope Clarification
- **MVP includes:** Single planet (Planet 1), full core loop, research, logistics puzzle rules, terrain tradeoffs, and offline progression.
- **MVP excludes:** Interplanetary progression, multi‑planet hazards, and resource transfer between worlds.

**Description:**
The central AI structure visually transforms as the tech tree advances, from crash-lander pod to planetary ring.

**User Story:**
> As a player who enjoys progression feedback, I want visual evolution so that my advancement feels tangible and impressive.

---

## User Interface

### Screens/Views

#### Screen 1: Planetary Grid View
**Purpose:** Main gameplay interface showing the planet surface, buildings, pipes, and resource flow.

**Components:**
- Grid-based terrain map
- Building placement interface
- Resource flow visualization
- Drone activity indicators
- Visible research button in the HUD (plus hotkey)

**User Actions:**
- Drag to place buildings and pipes -> Auto-routing with visual feedback
- Click terrain tiles -> Show utilization options
- Hover over buildings -> Display stats and production rates
- Research button (and hotkey) -> Open neural network view

---

#### Screen 2: Research Neural Network
**Purpose:** Visual research tree interface for unlocking new technologies and capabilities.

**Components:**
- Glowing neural network visualization
- Node connection lines
- Research progress indicators
- Data resource counter
- Technology descriptions

**User Actions:**
- Click unlocked nodes -> Research new technology
- Hover nodes -> Show requirements and effects
- View network expansion -> Track AI intelligence growth

---

#### Screen 3: Interplanetary Overview
**Purpose:** Solar system view showing conquered planets and resource transfer options.

**Components:**
- Planet selection interface
- Mass driver controls
- Resource transfer queues
- Planetary status indicators
- Ascension progress meter

**User Actions:**
- Select planet -> Travel to planet surface
- Configure mass drivers -> Set up resource exports
- View planetary bonuses -> Check interplanetary effects

---

#### Screen 4: Main Menu
**Purpose:** Game entry point with save management and settings.

**Components:**
- New Game / Continue options
- Save file management
- Settings panel
- Achievement/progress display

**User Actions:**
- Start new game -> Initialize first planet
- Load save -> Resume existing game
- Access settings -> Modify game preferences

---

### Design Guidelines

#### Color Palette
| Name | Hex | Usage |
|------|-----|-------|
| Primary | #00D9FF | AI Core, high-tech elements |
| Secondary | #666666 | Buildings, infrastructure |
| Accent | #FF6B35 | Resources, warnings |
| Background | #0A0A0A | Space/planet surface |
| Surface | #1A1A1A | UI panels, menus |
| Success | #4CAF50 | Positive feedback |
| Warning | #FF9800 | Hazards, alerts |
| Error | #F44336 | Critical issues |

#### Typography
- **Headings:** Roboto Bold, 24-48px, #FFFFFF
- **Body:** Roboto Regular, 16-20px, #CCCCCC
- **UI Labels:** Roboto Medium, 14px, #AAAAAA
- **Code/Numbers:** Roboto Mono, 12-16px, #00D9FF

---

## Technical Requirements

### Platform
- **Primary:** WebGL (WASM) for browser-based gameplay
- **Secondary:** Windows native executable
- **Mobile:** iOS Safari, Android Chrome (WebGL compatible)
- **Minimum Browser Requirements:** WebGL 2.0 support, modern JavaScript ES6+

### Performance
- **Load Time:** < 5 seconds initial load on modern hardware
- **Frame Rate:** 60 FPS target, 30 FPS minimum acceptable
- **Memory Usage:** < 200MB RAM for web version, < 100MB for native
- **Storage:** < 50MB save file size for complete game progress
- **Offline Simulation:** Support up to 4 hours of simulated gameplay

### Security
- **Data Protection:** Local save files only, no server-side storage required
- **Privacy:** No user data collection or tracking
- **Code Security:** Open source with regular dependency updates

### Data
- **Storage:** JSON-based save system for cross-platform compatibility
- **Persistence:** All progress maintained between sessions
- **Backup:** Manual save file export/import capability
- **Data Retention:** Indefinite local storage of player progress

---

## Constraints & Assumptions

### Constraints
- **Technology Stack:** Must use Rust and Macroquad engine for WebGL compatibility
- **Platform Limitations:** WebGL constraints on memory usage and shader complexity
- **Development Scope:** Single developer project with limited resources
- **Timeline:** MVP completion within 3-6 months
- **Target Platforms:** Web-first with native Windows support

### Assumptions
- **User Technical Requirements:** Modern browsers with WebGL 2.0 support available
- **Performance Expectations:** Target hardware can run 60 FPS WebGL applications
- **Save File Management:** Users understand basic file export/import for backups
- **Internet Connectivity:** Game functions primarily offline with optional web deployment

### Out of Scope
- Multiplayer functionality or server-side features
- Advanced 3D graphics or complex shader effects
- Mobile app store distribution (web-only deployment)
- Localization beyond English
- In-app purchases or monetization features

---

## Glossary

| Term | Definition |
|------|------------|
| Core | The central AI structure that processes resources and evolves |
| Conduit | Resource transportation pipes that connect buildings |
| Drone Swarm | Worker units that automatically carry resources along pipes |
| Logistics Puzzle | Spatial challenges created by non-overlapping pipe constraints |
| Mass Driver | Interplanetary resource transfer system |
| Neural Network | Visual research tree representing AI intelligence growth |
| Planetary Consumption | The process of harvesting a planet's resources to build a Seed Ship |
| Seed Ship | Spacecraft built from planetary resources to colonize new worlds |
| Terrain Utilization | Strategic choice between harvesting or preserving terrain features |

---

## Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-01-24 | AI Assistant | Initial PRD creation based on GDD specifications |
| 0.1 | 2026-01-24 | AI Assistant | Draft structure and core feature identification |
| 1.1 | 2026-01-24 | AI Assistant | Clarify MVP scope to Planet 1; move interplanetary to post‑MVP; require visible research button |

---

*Last updated: 2026-01-24*
