//! Spatial calculations, terrain, buildings, and pathfinding

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// Grid position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl GridPos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Check if position is within grid bounds
    pub fn in_bounds(&self, width: u32, height: u32) -> bool {
        self.x >= 0 && self.y >= 0 && (self.x as u32) < width && (self.y as u32) < height
    }

    /// Get adjacent positions (4-directional)
    pub fn neighbors(&self) -> [GridPos; 4] {
        [
            GridPos::new(self.x - 1, self.y),
            GridPos::new(self.x + 1, self.y),
            GridPos::new(self.x, self.y - 1),
            GridPos::new(self.x, self.y + 1),
        ]
    }

    /// Manhattan distance to another position
    pub fn distance(&self, other: GridPos) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }

    /// Convert to array index for flat storage
    pub fn to_index(&self, width: u32) -> usize {
        (self.y as u32 * width + self.x as u32) as usize
    }

    /// Create from array index
    pub fn from_index(index: usize, width: u32) -> Self {
        Self {
            x: (index as u32 % width) as i32,
            y: (index as u32 / width) as i32,
        }
    }
}

/// Terrain types that affect gameplay
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerrainType {
    Empty,      // Buildable ground
    Mountain,   // Can harvest for iron or place wind turbine
    Forest,     // Can harvest for biomass or keep as pollution buffer
    Water,      // Cannot build, may provide cooling
    Rough,      // Difficult to build on (result of harvesting)
    Void,       // Unbuildable gap (volcanic terrain)
}

impl TerrainType {
    /// Whether buildings can be placed on this terrain
    pub fn is_buildable(&self) -> bool {
        matches!(self, TerrainType::Empty | TerrainType::Rough)
    }

    /// Whether this terrain can be harvested
    pub fn is_harvestable(&self) -> bool {
        matches!(self, TerrainType::Mountain | TerrainType::Forest)
    }

    /// Get harvest rewards (minerals, biomass)
    pub fn harvest_rewards(&self) -> (f32, f32) {
        match self {
            TerrainType::Mountain => (50.0, 0.0),  // Iron/minerals
            TerrainType::Forest => (0.0, 30.0),   // Biomass
            _ => (0.0, 0.0),
        }
    }

    /// Get terrain after harvesting
    pub fn harvested(&self) -> TerrainType {
        match self {
            TerrainType::Mountain => TerrainType::Rough,
            TerrainType::Forest => TerrainType::Empty,
            _ => *self,
        }
    }

    /// Get preservation bonus description
    pub fn preservation_bonus(&self) -> Option<&'static str> {
        match self {
            TerrainType::Mountain => Some("+100% Wind Turbine efficiency"),
            TerrainType::Forest => Some("Pollution buffer (future)"),
            _ => None,
        }
    }

    /// Display name
    pub fn name(&self) -> &'static str {
        match self {
            TerrainType::Empty => "Ground",
            TerrainType::Mountain => "Mountain",
            TerrainType::Forest => "Forest",
            TerrainType::Water => "Water",
            TerrainType::Rough => "Rough Ground",
            TerrainType::Void => "Void",
        }
    }
}

impl Default for TerrainType {
    fn default() -> Self {
        TerrainType::Empty
    }
}

/// Building types that can be placed on the grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildingType {
    Core,       // Central AI structure - receives resources
    Drill,      // Extracts minerals, spawns drones
    Conduit,    // Connects buildings for resource flow
    Bridge,     // Allows conduit crossings (overlay)
    PowerNode,  // Extends power grid
    WindTurbine, // Generates power (bonus on mountains)
    ServerBank, // Generates data, consumes power
}

impl BuildingType {
    /// Resource cost to build
    pub fn cost(&self) -> (f32, f32) {
        // Returns (minerals, energy)
        match self {
            BuildingType::Core => (0.0, 0.0),        // Free (starting building)
            BuildingType::Drill => (20.0, 10.0),
            BuildingType::Conduit => (5.0, 2.0),
            BuildingType::Bridge => (15.0, 5.0),
            BuildingType::PowerNode => (15.0, 5.0),
            BuildingType::WindTurbine => (25.0, 15.0),
            BuildingType::ServerBank => (40.0, 30.0),
        }
    }

    /// Display name for UI
    pub fn name(&self) -> &'static str {
        match self {
            BuildingType::Core => "Core",
            BuildingType::Drill => "Drill",
            BuildingType::Conduit => "Conduit",
            BuildingType::Bridge => "Bridge",
            BuildingType::PowerNode => "Power Node",
            BuildingType::WindTurbine => "Wind Turbine",
            BuildingType::ServerBank => "Server Bank",
        }
    }

    /// Keyboard shortcut for quick selection
    pub fn hotkey(&self) -> Option<char> {
        match self {
            BuildingType::Drill => Some('1'),
            BuildingType::Conduit => Some('2'),
            BuildingType::Bridge => Some('6'),
            BuildingType::PowerNode => Some('3'),
            BuildingType::WindTurbine => Some('4'),
            BuildingType::ServerBank => Some('5'),
            BuildingType::Core => None,
        }
    }
}

/// A building placed on the grid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Building {
    pub building_type: BuildingType,
    pub position: GridPos,
    pub powered: bool,
    pub efficiency: f32,  // 0.0 to 1.0+
    pub connected_to_core: bool,  // For logistics validation
}

impl Building {
    pub fn new(building_type: BuildingType, position: GridPos) -> Self {
        let is_core = building_type == BuildingType::Core;
        Self {
            building_type,
            position,
            powered: is_core,
            efficiency: 1.0,
            connected_to_core: is_core,
        }
    }

    /// Check if this building transmits power
    pub fn transmits_power(&self) -> bool {
        matches!(
            self.building_type,
            BuildingType::Core | BuildingType::Conduit | BuildingType::PowerNode
        )
    }

    /// Check if this building generates power
    pub fn generates_power(&self) -> bool {
        matches!(
            self.building_type,
            BuildingType::Core | BuildingType::WindTurbine
        )
    }

    /// Check if this building consumes power
    pub fn consumes_power(&self) -> bool {
        matches!(
            self.building_type,
            BuildingType::Drill | BuildingType::ServerBank
        )
    }
}

/// A single tile on the grid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub terrain: TerrainType,
    pub building: Option<Building>,
    pub revealed: bool,  // For fog of war / expansion
    #[serde(default)]
    pub bridge: bool,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            terrain: TerrainType::Empty,
            building: None,
            revealed: false,
            bridge: false,
        }
    }
}

/// The game grid containing all tiles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    tiles: Vec<Tile>,
}

impl Grid {
    const POWER_REPEATER_RANGE: u32 = 6;
    /// Create a new grid with default empty tiles
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            tiles: vec![Tile::default(); size],
        }
    }

    /// Create a grid with procedural terrain
    pub fn new_with_terrain(width: u32, height: u32, seed: u64) -> Self {
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;

        let mut rng = StdRng::seed_from_u64(seed);
        let size = (width * height) as usize;
        let mut tiles = Vec::with_capacity(size);

        let center_x = width as i32 / 2;
        let center_y = height as i32 / 2;

        for i in 0..size {
            let pos = GridPos::from_index(i, width);
            let dist_from_center = pos.distance(GridPos::new(center_x, center_y));

            // Terrain distribution based on distance from center
            let terrain = if dist_from_center <= 2 {
                TerrainType::Empty // Clear area around Core
            } else {
                let roll: f32 = rng.gen();
                if roll < 0.6 {
                    TerrainType::Empty
                } else if roll < 0.75 {
                    TerrainType::Mountain
                } else if roll < 0.9 {
                    TerrainType::Forest
                } else if roll < 0.95 {
                    TerrainType::Water
                } else {
                    TerrainType::Void
                }
            };

            // Reveal tiles near center
            let revealed = dist_from_center <= 4;

            tiles.push(Tile {
                terrain,
                building: None,
                revealed,
                bridge: false,
            });
        }

        Self { width, height, tiles }
    }

    /// Get tile at position (returns None if out of bounds)
    pub fn get(&self, pos: GridPos) -> Option<&Tile> {
        if pos.in_bounds(self.width, self.height) {
            Some(&self.tiles[pos.to_index(self.width)])
        } else {
            None
        }
    }

    /// Get mutable tile at position
    pub fn get_mut(&mut self, pos: GridPos) -> Option<&mut Tile> {
        if pos.in_bounds(self.width, self.height) {
            let index = pos.to_index(self.width);
            Some(&mut self.tiles[index])
        } else {
            None
        }
    }

    /// Check if a building can be placed at position
    pub fn can_place_building(&self, pos: GridPos, building_type: BuildingType) -> bool {
        if let Some(tile) = self.get(pos) {
            if !tile.revealed {
                return false;
            }
            if tile.building.is_some() {
                if building_type == BuildingType::Bridge {
                    if let Some(ref building) = tile.building {
                        return building.building_type == BuildingType::Conduit && !tile.bridge;
                    }
                }
                return false;
            }
            // Conduits cannot overlap any existing building and must be on buildable terrain
            if building_type == BuildingType::Conduit {
                return tile.terrain.is_buildable();
            }
            if building_type == BuildingType::Bridge {
                return false;
            }
            // Special case: Wind turbines can go on mountains
            if building_type == BuildingType::WindTurbine {
                return matches!(tile.terrain, TerrainType::Empty | TerrainType::Mountain);
            }
            tile.terrain.is_buildable()
        } else {
            false
        }
    }

    /// Place a building at position
    pub fn place_building(&mut self, pos: GridPos, building_type: BuildingType) -> bool {
        if !self.can_place_building(pos, building_type) {
            return false;
        }

        if let Some(tile) = self.get_mut(pos) {
            if building_type == BuildingType::Bridge {
                tile.bridge = true;
                return true;
            }
            let mut building = Building::new(building_type, pos);

            // Wind turbines on mountains get efficiency bonus
            if building_type == BuildingType::WindTurbine
                && tile.terrain == TerrainType::Mountain
            {
                building.efficiency = 2.0; // +100% bonus
            }

            tile.building = Some(building);
            true
        } else {
            false
        }
    }

    /// Remove a building at position
    pub fn remove_building(&mut self, pos: GridPos) -> Option<Building> {
        if let Some(tile) = self.get_mut(pos) {
            tile.bridge = false;
            tile.building.take()
        } else {
            None
        }
    }

    /// Reveal tiles around a position
    pub fn reveal_around(&mut self, center: GridPos, radius: u32) {
        for dy in -(radius as i32)..=(radius as i32) {
            for dx in -(radius as i32)..=(radius as i32) {
                let pos = GridPos::new(center.x + dx, center.y + dy);
                if pos.in_bounds(self.width, self.height) {
                    if center.distance(pos) <= radius {
                        if let Some(tile) = self.get_mut(pos) {
                            tile.revealed = true;
                        }
                    }
                }
            }
        }
    }

    /// Find the Core building position
    pub fn find_core(&self) -> Option<GridPos> {
        for (i, tile) in self.tiles.iter().enumerate() {
            if let Some(ref building) = tile.building {
                if building.building_type == BuildingType::Core {
                    return Some(GridPos::from_index(i, self.width));
                }
            }
        }
        None
    }

    /// Get all buildings of a specific type
    pub fn find_buildings(&self, building_type: BuildingType) -> Vec<GridPos> {
        self.tiles
            .iter()
            .enumerate()
            .filter_map(|(i, tile)| {
                tile.building
                    .as_ref()
                    .filter(|b| b.building_type == building_type)
                    .map(|_| GridPos::from_index(i, self.width))
            })
            .collect()
    }

    /// Iterator over all tiles with positions
    pub fn iter_tiles(&self) -> impl Iterator<Item = (GridPos, &Tile)> {
        self.tiles
            .iter()
            .enumerate()
            .map(move |(i, tile)| (GridPos::from_index(i, self.width), tile))
    }

    /// Find a conduit path that avoids blocked tiles
    pub fn find_conduit_path(&self, from: GridPos, to: GridPos) -> Option<Vec<GridPos>> {
        if from == to {
            return Some(Vec::new());
        }

        let is_passable = |pos: GridPos, grid: &Grid| {
            if let Some(tile) = grid.get(pos) {
                if !tile.revealed {
                    return false;
                }
                if !tile.terrain.is_buildable() {
                    return false;
                }
                match tile.building.as_ref() {
                    None => true,
                    Some(building) => building.building_type == BuildingType::Conduit,
                }
            } else {
                false
            }
        };

        let mut queue: VecDeque<GridPos> = VecDeque::new();
        let mut came_from: HashMap<GridPos, GridPos> = HashMap::new();
        queue.push_back(from);

        while let Some(current) = queue.pop_front() {
            for neighbor in current.neighbors() {
                if !neighbor.in_bounds(self.width, self.height) {
                    continue;
                }
                if came_from.contains_key(&neighbor) || neighbor == from {
                    continue;
                }
                if !is_passable(neighbor, self) {
                    continue;
                }
                came_from.insert(neighbor, current);
                if neighbor == to {
                    queue.clear();
                    break;
                }
                queue.push_back(neighbor);
            }
        }

        if !came_from.contains_key(&to) {
            return None;
        }

        let mut reversed = Vec::new();
        let mut current = to;
        while current != from {
            reversed.push(current);
            current = *came_from.get(&current).unwrap();
        }
        reversed.reverse();
        Some(reversed)
    }

    /// Count total buildings on the grid
    pub fn total_buildings(&self) -> usize {
        self.tiles.iter().filter(|tile| tile.building.is_some()).count()
    }

    /// Update power grid connectivity using flood fill from Core
    pub fn update_power_grid(&mut self) {
        // First, reset all buildings to unpowered
        for tile in &mut self.tiles {
            if let Some(ref mut building) = tile.building {
                building.powered = building.building_type == BuildingType::Core;
                building.connected_to_core = building.building_type == BuildingType::Core;
            }
        }

        // Find Core position
        let core_pos = match self.find_core() {
            Some(pos) => pos,
            None => return,
        };

        // Flood fill from Core through power-transmitting buildings with repeater range
        let mut best_distance: std::collections::HashMap<GridPos, u32> = std::collections::HashMap::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((core_pos, 0u32));
        best_distance.insert(core_pos, 0u32);

        while let Some((pos, distance_since_repeater)) = queue.pop_front() {
            // Mark this building as connected and powered
            if let Some(tile) = self.get_mut(pos) {
                if let Some(ref mut building) = tile.building {
                    building.connected_to_core = true;
                    building.powered = true;
                }
            }

            let next_distance = if let Some(tile) = self.get(pos) {
                if let Some(ref building) = tile.building {
                    if matches!(building.building_type, BuildingType::Core | BuildingType::PowerNode) {
                        0
                    } else {
                        distance_since_repeater + 1
                    }
                } else {
                    distance_since_repeater + 1
                }
            } else {
                distance_since_repeater + 1
            };

            // Check neighbors
            for neighbor in pos.neighbors() {
                if !neighbor.in_bounds(self.width, self.height) {
                    continue;
                }

                // Check if neighbor has a power-transmitting building
                if let Some(tile) = self.get(neighbor) {
                    if let Some(ref building) = tile.building {
                        if building.transmits_power() {
                            if next_distance <= Self::POWER_REPEATER_RANGE {
                                let should_visit = match best_distance.get(&neighbor) {
                                    Some(existing) => next_distance < *existing,
                                    None => true,
                                };
                                if should_visit {
                                    best_distance.insert(neighbor, next_distance);
                                    queue.push_back((neighbor, next_distance));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Now mark buildings adjacent to powered conduits/nodes as powered
        let powered_positions: Vec<GridPos> = self.tiles
            .iter()
            .enumerate()
            .filter_map(|(i, tile)| {
                tile.building.as_ref()
                    .filter(|b| b.powered && b.transmits_power())
                    .map(|_| GridPos::from_index(i, self.width))
            })
            .collect();

        for powered_pos in powered_positions {
            for neighbor in powered_pos.neighbors() {
                if let Some(tile) = self.get_mut(neighbor) {
                    if let Some(ref mut building) = tile.building {
                        if !building.transmits_power() {
                            building.powered = true;
                            building.connected_to_core = true;
                        }
                    }
                }
            }
        }
    }

    /// Check if position is adjacent to a powered building
    pub fn is_adjacent_to_power(&self, pos: GridPos) -> bool {
        for neighbor in pos.neighbors() {
            if let Some(tile) = self.get(neighbor) {
                if let Some(ref building) = tile.building {
                    if building.powered && building.transmits_power() {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Get total power generation
    pub fn total_power_generation(&self) -> f32 {
        self.tiles
            .iter()
            .filter_map(|tile| tile.building.as_ref())
            .filter(|b| b.powered && b.generates_power())
            .map(|b| match b.building_type {
                BuildingType::Core => 10.0,
                BuildingType::WindTurbine => 5.0 * b.efficiency,
                _ => 0.0,
            })
            .sum()
    }

    /// Get total power consumption
    pub fn total_power_consumption(&self) -> f32 {
        self.tiles
            .iter()
            .filter_map(|tile| tile.building.as_ref())
            .filter(|b| b.powered && b.consumes_power())
            .map(|b| match b.building_type {
                BuildingType::Drill => 2.0,
                BuildingType::ServerBank => 5.0,
                _ => 0.0,
            })
            .sum()
    }

    /// Calculate net power (generation - consumption)
    pub fn net_power(&self) -> f32 {
        self.total_power_generation() - self.total_power_consumption()
    }
}
