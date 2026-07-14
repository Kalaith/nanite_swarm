# Nanite Swarm — Prototype → Commercial Roadmap

*Generated 2026-07-14 from a full code audit (~7,100 LoC) compared against `gdd.md`.*

## Where the prototype actually stands

**Genuinely solid foundations:** the single-planet automation loop works — 10 buildings, drone hauling, power flood-fill with repeater range, a fully-implemented dust/degradation system, the mountain/forest harvest-vs-utilize dilemma, a 15-node visual research graph, directives, offline/idle progress with the 4-hour battery + hibernation model, save/load on both native and WASM, a procedural art pipeline (~56 generated sprites), placement previews, particles, and per-state drone visuals. CI, clippy-clean, headless screenshot harness.

**The honest gap:** the game's *pitch* — the interplanetary supply chain and solar campaign — is a facade, there is no win or loss condition, the logistics "spaghetti puzzle" at the heart of the GDD doesn't mechanically exist, a third of the research tree does nothing, and there is zero audio. What exists is a good vertical slice of the first 30 minutes on one planet.

---

## 1. Core game design — finish the fantasy

### 1.1 Win/loss and the ascension loop (nothing exists)
- [ ] **Seed Ship**: the GDD's per-planet win condition is entirely absent (no reference in code). Needs: multi-stage megastructure construction, large resource sinks, launch sequence, per-planet victory.
- [ ] **Victory / game-over states**: `GamePhase` has only MainMenu/Playing/Research/Interplanetary/Settings. No campaign-complete state, no terminal failure.
- [ ] **Real collapse pressure**: power collapse today is a 20-second timeout with a data penalty — annoying, not threatening. Decide whether infrastructure collapse can actually end a run, and design the difficulty curve around it.
- [ ] **Core evolution as mechanics**: 5 visual core stages render by progress, but there is no mechanical stage progression (Crash-Lander → Fortress → Space Elevator → Planetary Ring per GDD §5). Tie stages to research/throughput milestones with new capabilities per stage.

### 1.2 Interplanetary meta-layer (currently a facade)
- [ ] **Persist planet state across travel** — `main.rs:210` ("Load planet state here (for now, create new)") discards the current planet and generates a fresh random one. The GDD's core promise ("Planet 1 does not disappear") is inverted.
- [ ] **Background simulation of colonized planets** — scheduled-tick or summary simulation so left-behind worlds keep producing (this is the "Interplanetary Supply Chain").
- [ ] **Mass Drivers as real gameplay** — today the `mass_driver` tech + 100 minerals flips a boolean. Needs: a Mass Driver building, export schedules ("500 steel/min from Mars to Saturn"), transit time, receiving-side landing pads.
- [ ] **Persist meta-state** — `colonized_planets`, `current_planet_index`, and the active directive live on `Game` and are lost on save/reload.
- [ ] **Per-planet generation rules** — all 5 planets currently call the same `PlanetState::new(24, 24, seed)`; difficulty exists only as flavor strings.

### 1.3 Planet hazards (Zones 1–3 of the GDD)
- [ ] Dust (Zone 1 / Mars) is the only implemented hazard — and it's good. Use it as the template.
- [ ] **Zone 2 (Venus)**: acid rain degrading standard conduits, Ceramic Plating / Shield Generator counters, void-heavy volcanic terrain generation.
- [ ] **Zone 3 (Cryo)**: freeze slowing drones 50%, Heater Nodes along the network, no solar/wind power.
- [ ] **Per-planet power constraints**: which generators work where (no solar on Venus, geothermal infinite, fusion-only on cryo worlds) — this is what makes each planet a different puzzle.
- [ ] **Heat mechanic**: GDD says Server Banks generate Heat; no thermal system exists anywhere. Water tiles have a "may provide cooling" comment with no logic — a natural pairing.

### 1.4 Research that actually works
- [ ] **5 of 15 techs are no-ops**: `efficient_drills` (+50% output), `drone_capacity` (+100%), `power_efficiency` (−25% consumption), `advanced_research` (+50% data), `neural_expansion` — their ids are never read outside the tree UI. Players *will* notice paid upgrades doing nothing.
- [ ] **Build a modifier/stat system** so tech effects are declared in `research.json` and applied generically, replacing the stringly-typed `unlocked_techs.contains("...")` pattern that caused these silent no-ops.
- [ ] **Expand the tree** — 15 nodes is 1–2 hours of progression; a commercial idle/logistics game needs 60–100+ across multiple planets/eras, with per-planet branches (hazard counters as research).

### 1.5 The logistics puzzle — the biggest design decision
- [ ] The GDD's difficulty pillar ("The Spaghetti": strict non-overlapping pipes, expensive crossings, physical resource flow) **does not mechanically exist**. Conduits are power-only; drones BFS over raw terrain, ignore conduits and buildings entirely, never collide, and never re-path. Bridges are a bool flag that doesn't even transmit power. **Decide**: (a) make drones route along the conduit network with throughput limits and real crossings — the GDD's vision — or (b) formally redesign around free-flying drones with congestion/interference. Everything about content and balance downstream depends on this.
- [ ] Drone re-pathing when the grid changes; emit the existing-but-unused `DroneEvent::PathBlocked`; error-flag UX when a route breaks (GDD promises this).
- [ ] Congestion/throughput as a scaling pressure (currently 1 drone per drill, no interaction).
- [ ] **Resource deposits**: drills output a hardcoded 10 minerals/cycle from *any* tile — there are no ore veins/deposits, so placement is spatially meaningless. Deposit tiles with richness/depletion are genre table stakes and feed the terrain dilemma.

## 2. Content volume (biggest schedule item after design)

- [ ] **Production chains**: minerals currently teleport into one global pool; there are no intermediate products, recipes, or processing buildings (smelter → alloy → components). Without chains there's nothing to optimize. This multiplies with building count.
- [ ] **Buildings**: 10 → roughly 25–40 across tiers (processing, logistics, per-planet hazard counters, megastructure parts).
- [ ] **Resources**: 4 (Minerals/Energy/Data/Biomass) → tiered materials to support chains and mass-driver strategy.
- [ ] **Map scale and variety**: fixed 24×24 with one generator recipe (60/15/15/5/5 distribution). Larger maps, per-planet generators, landmark features. (Requires camera work — §5.)
- [ ] **Objectives**: 4 directive kinds cycling `tier % 4`; 4 hardcoded achievements. Needs a real objective/milestone system and 30–50 achievements for platform integration.
- [ ] **Narrative dressing**: the GDD's "indifferent optimizer" tone exists nowhere in game text — cheap, high-impact flavor via directives, research descriptions, and planet-arrival vignettes.

## 3. Simulation architecture (unblocks everything above)

- [ ] **Fixed timestep**: `get_frame_time()` feeds the sim directly — behavior is frame-rate dependent, and offline sim already uses different step sizes (60s chunks). An accumulator + fixed tick makes native/WASM/offline/background-planet simulation consistent.
- [ ] **Sim/view separation**: pull simulation out of `screens/` (README already flags this) so planetary + interplanetary + background planets share one engine. Prerequisite for the meta-layer.
- [ ] **Determinism**: terrain RNG is seeded, but timers/float accumulation aren't tick-quantized; add the deterministic snapshot tests the README calls for (terrain harvest, power failure, drone routing, research, collapse).
- [ ] **Balance data hygiene**: drill output is hardcoded `10.0/cycle` while the HUD displays the unused config value `2.0`; `conduit_throughput` and `core_power_consumption` are dead config fields; dust rates, collapse timers (60s/120s/20s), repeater range (6), refund ratio (0.5), battery (4h), mass-driver cost (100) are all Rust constants. Move to validated JSON fixtures (README goal) so balancing doesn't require recompiles.
- [ ] **Robust data loading**: `game_data().building(id)` panics on missing ids — validate at load with real error messages.

## 4. Save system

- [ ] **Autosave** (interval + on-quit + on-travel). Today: manual save or entering the menu only.
- [ ] **Save versioning + migration** — no schema version field exists; the first post-release balance patch will corrupt or silently mangle old saves.
- [ ] **Multiple slots** (single `save.json` today) and corruption recovery (backup rotation).
- [ ] **Persist everything meta**: colonized planets, current planet, directive, settings, tutorial progress — all currently lost on reload.
- [ ] **Offline-progress hardening**: clock-tamper guard, hard caps, and an offline-earnings summary screen (the banner exists; make it a report).
- [ ] Steam Cloud / platform save sync when platform work lands (§8).

## 5. UX / UI

- [ ] **Camera pan + zoom**: none exists (fixed 28px grid, no `Camera2D`) — mandatory for any map bigger than 24×24.
- [ ] **Pause and game speed**: no pause state exists, yet the HUD advertises "PAUSE Space" and shows ± speed buttons whose return values are ignored ("1.0x" hardcoded). Idle games need 1×/2×/4×.
- [ ] **Wire or remove decorative controls**: bottom-bar hints for PAN, ZOOM, BOX SELECT (Shift+Drag), BUILD (B), DEMOLISH (X) are all unwired; the main-menu **Quit button is a no-op** (`MenuAction::Quit => {}`). Players read these as bugs.
- [ ] **Real tutorial**: current one is a 6-condition checklist shown as text, not persisted (resets every load), with no highlighting or interactive gating. First-session retention lives or dies here.
- [ ] **Notification/toast system**: alerts are ad-hoc banners; achievements unlock with zero feedback (only an N/total counter).
- [ ] **Settings that work and persist**: `ui_scale` is stored but never applied; audio sliders drive nothing (§6); the whole Settings struct is never saved. Add resolution/fullscreen (native), vsync, and keybind remapping.
- [ ] **Genre-standard build tools**: demolish mode, drag-demolish, building relocation, copy/blueprint stamps, undo.
- [ ] **Production statistics**: rates, consumption, net-flow graphs (the bottom-bar graph is currently decorative) — factory players expect this for diagnosing bottlenecks.

## 6. Audio (from absolute zero)

- [ ] There is **no audio system, no sound files, no `macroquad::audio` import** — the settings sliders are placebos. Needed: music (menu + gameplay layers that respond to swarm scale/collapse), ambient bed, and SFX for placement, demolition, harvest, drone delivery, research complete, directive complete, power-collapse alarm, achievement, and UI interaction. Evaluate `macroquad::audio` vs `kira` early — WASM audio has real constraints (autoplay policies, latency).

## 7. Art & presentation

- [ ] The procedural 28px pipeline (`build_graphics.rs`) is a clever cost-saver and fine as a base, but commercial "abstract industrial" still needs: **animation** (rotating drills, blinking servers, turbine spin — everything is static sprites today), a cohesive style/palette pass, screen juice (shake on collapse, harvest impact), day/night or atmosphere per planet.
- [ ] Core stages 3–4 visuals (Space Elevator tether, background Planetary Ring) — the GDD's signature image.
- [ ] **Marketing art**: key art, logo, animated store capsules, trailer. Only `catalog_thumbnail.png` exists.

## 8. Platforms & commercial plan

> **DECIDED (2026-07-14): Premium PC game at $5. Launch on itch.io first; port to Steam only if itch shows traction. Mobile is dropped** (removes touch input, phone HUD relayout, app-store compliance, and F2P design pressure from scope). The GDD's "PC & Mobile (Hybrid)" framing is superseded.

### Phase A — itch.io launch ($5)
- [ ] **itch.io release engineering**: Windows zip is already produced (`dist/nanite_swarm_windows.zip`); add itch page setup, `butler` push pipeline (integrate into `publish.ps1`), version stamping in-game so bug reports are traceable.
- [ ] **Decide the free-web-build question**: the game currently ships free on the WebHatchery portal (WASM). Selling at $5 alongside a free full web version undercuts the price — either take the portal build down, freeze it as a limited demo (e.g. first planet only), or make the itch page itself host a browser demo + paid download. Pick deliberately.
- [ ] **Remove/replace the Ko-fi widget and bug-report branding** for the paid distribution — a donate button inside a paid product reads badly.
- [ ] **itch page assets**: cover image (630×500), screenshots/GIFs, trailer optional-but-valuable at this price point, description copy.
- [ ] **Pricing hygiene**: $5 tier sets expectations — scope polish accordingly (a $5 game is forgiven missing localization; it is not forgiven a Quit button that does nothing).
- [ ] **Offline-battery model review**: the 4-hour battery/hibernation system was designed for mobile check-in retention. It still suits a desktop idle game, but tune it for PC session patterns (and make the offline-earnings report satisfying — it's now a selling point, not a retention hook).

### Phase B — Steam (contingent on itch traction)
- [ ] Steamworks integration: achievements (needs the expanded achievement set from §2), cloud saves, rich presence, overlay compatibility.
- [ ] Store page + wishlist campaign well before launch; demo build (Next Fest) as the discovery vehicle.
- [ ] **Steam Deck**: controller/gamepad support (none exists) and Deck verification — defer entirely until Phase B is confirmed, but avoid UI decisions in M3 that assume mouse-only (e.g. keep hit targets generous).
- [ ] Steam Cloud save sync (extends §4's save versioning work).

### Cross-cutting (both phases)
- [ ] **Localization**: 100% hardcoded English strings across Rust and JSON, no string-table system. For a $5 itch launch this can slip to post-launch, but externalize strings *before* M3's content expansion doubles the count — retrofitting later is far more expensive. CJK/Cyrillic font pipeline only if Steam happens (where localization measurably moves sales).
- [ ] **Accessibility**: state is encoded almost entirely in color (drone states, power ±, valid/invalid placement, research status) — colorblind-safe shapes/patterns, working text scaling, reduced-motion option, remappable keys.

## 9. Engineering quality

- [ ] **Test coverage**: exactly **1 unit test** exists (offline hibernation). Priority targets: power flood-fill + repeater range, drone pathfinding/re-path, placement rules, harvest consequences, research unlock/effects, save round-trip + migration, offline sim, collapse thresholds. The headless capture harness already covers visual smoke tests — wire it into CI scenes.
- [ ] **File-size violations** (repo hard limit 800): `screens/planetary_view.rs` **2,053** (layout + rendering + all input + entire HUD in one file, with a 950-line function), `state/game_state.rs` 852, `engine/grid_engine.rs` 786. Do the planetary_view split (hud/, grid_render/, input/) before UI work makes it worse.
- [ ] **Telemetry & crash reporting** (opt-in): balance funnels (where players stall/quit), error reporting beyond the current manual bug-report widget.
- [ ] **Performance budget**: fine at 24×24; multi-planet background sim + larger maps + hundreds of drones need profiling targets (especially WASM).

## 10. Launch operations

- [ ] **Playtesting program**: closed alpha → balance iteration loop using telemetry; the game has never been balanced beyond the author's play. itch makes this easy — a private/password page or first-week discount cohort works as the alpha channel.
- [ ] **QA matrix**: Windows versions × GPUs (+ browsers if a web demo ships); save-compat testing per patch. Mobile devices dropped from scope.
- [ ] **Legal/compliance**: privacy policy (required once telemetry exists), EULA, itch tax/payout setup; Steam compliance only in Phase B.
- [ ] **Community & marketing**: itch devlog cadence (itch's devlog feed is its main discovery mechanism), Discord, GIF-forward social posts (automation games market well as timelapse GIFs), press/creator kit. Steam wishlist beats move to Phase B.
- [ ] **Traction gate for Steam**: define the threshold up front (e.g. X sales / Y collections / follower growth in the first 1–2 months on itch) so the Phase B decision is data-driven, not vibes.
- [ ] **Post-launch plan**: patch cadence, content roadmap (later solar zones are natural free updates at this price — they also generate itch devlog beats).

---

## Suggested milestone sequencing

| Milestone | Theme | Contents | Rough scale |
|---|---|---|---|
| **M1 — "It's a game"** | Complete the loop | Win/loss + Seed Ship, fix no-op techs (modifier system), fixed timestep, autosave + save versioning, wire/remove decorative UI, pause | 4–6 weeks |
| **M2 — "The hook is real"** | Meta-layer | Planet persistence + background sim, real mass drivers/exports, hazards for 3 planet types, logistics-depth decision implemented, deposits | 6–10 weeks |
| **M3 — "Content & feel"** | Depth + presentation | Production chains, tree to 60+, bigger maps + camera, audio from zero, real tutorial, animation/juice, planetary_view refactor | 8–12 weeks |
| **M4 — "itch launch"** | Commercial | Accessibility basics, QA matrix, itch page + butler pipeline, web-demo decision, playtesting/balance, launch ops | 4–6 weeks |
| **M5 — "Steam port"** *(contingent)* | Phase B | Steamworks, controller/Deck, localization, store page + wishlists, Next Fest demo | 6–8 weeks |

With mobile dropped, M1–M4 lands at roughly **5–7 months** to a $5 itch release — consistent with the workspace's own `standing.md` estimate of 3–5 months remaining plus launch operations. M5 only happens if itch traction clears the gate defined in §10.
