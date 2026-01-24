//! Neural network and tech progression

use serde::{Deserialize, Serialize};

/// Research node in the neural network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchNode {
    pub id: String,
    pub name: String,
    pub description: String,
    pub data_cost: f32,
    pub prerequisites: Vec<String>,
    pub position: (f32, f32),  // Visual position in neural network
}

impl ResearchNode {
    pub fn new(id: &str, name: &str, description: &str, cost: f32, prereqs: Vec<&str>, pos: (f32, f32)) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            data_cost: cost,
            prerequisites: prereqs.into_iter().map(|s| s.to_string()).collect(),
            position: pos,
        }
    }
}

/// The research tree containing all nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchTree {
    pub nodes: Vec<ResearchNode>,
}

impl Default for ResearchTree {
    fn default() -> Self {
        Self {
            nodes: vec![
                // Core node (always unlocked)
                ResearchNode::new(
                    "core",
                    "AI Core",
                    "Central processing unit",
                    0.0,
                    vec![],
                    (0.0, 0.0),
                ),
                // Tier 1 - Basic techs (require core)
                ResearchNode::new(
                    "basic_mining",
                    "Basic Mining",
                    "Enables drill construction",
                    0.0, // Free - starting tech
                    vec!["core"],
                    (-1.5, -1.0),
                ),
                ResearchNode::new(
                    "power_grid",
                    "Power Grid",
                    "Enables conduits and power nodes",
                    10.0,
                    vec!["core"],
                    (1.5, -1.0),
                ),
                ResearchNode::new(
                    "data_processing",
                    "Data Processing",
                    "Enables server banks",
                    15.0,
                    vec!["core"],
                    (0.0, -1.5),
                ),
                // Tier 2 - Advanced techs
                ResearchNode::new(
                    "efficient_drills",
                    "Efficient Drills",
                    "+50% drill output",
                    25.0,
                    vec!["basic_mining"],
                    (-2.0, -2.0),
                ),
                ResearchNode::new(
                    "drone_capacity",
                    "Drone Capacity",
                    "+100% drone carry capacity",
                    30.0,
                    vec!["basic_mining"],
                    (-1.0, -2.0),
                ),
                ResearchNode::new(
                    "wind_power",
                    "Wind Power",
                    "Enables wind turbines",
                    20.0,
                    vec!["power_grid"],
                    (2.0, -2.0),
                ),
                ResearchNode::new(
                    "power_efficiency",
                    "Power Efficiency",
                    "-25% power consumption",
                    35.0,
                    vec!["power_grid"],
                    (1.0, -2.0),
                ),
                ResearchNode::new(
                    "advanced_research",
                    "Advanced Research",
                    "+50% data generation",
                    40.0,
                    vec!["data_processing"],
                    (0.0, -2.5),
                ),
                // Tier 3 - Specializations
                ResearchNode::new(
                    "mass_driver",
                    "Mass Driver",
                    "Enables interplanetary transport",
                    100.0,
                    vec!["efficient_drills", "power_efficiency"],
                    (0.0, -3.5),
                ),
                ResearchNode::new(
                    "neural_expansion",
                    "Neural Expansion",
                    "Unlocks advanced AI capabilities",
                    150.0,
                    vec!["advanced_research", "mass_driver"],
                    (0.0, -4.5),
                ),
            ],
        }
    }
}

impl ResearchTree {
    /// Get a node by ID
    pub fn get_node(&self, id: &str) -> Option<&ResearchNode> {
        self.nodes.iter().find(|n| n.id == id)
    }

    /// Check if a node can be researched
    pub fn can_research(&self, id: &str, unlocked: &[String], available_data: f32) -> bool {
        if let Some(node) = self.get_node(id) {
            // Already unlocked?
            if unlocked.contains(&node.id) {
                return false;
            }
            // Prerequisites met?
            if !node.prerequisites.iter().all(|p| unlocked.contains(p)) {
                return false;
            }
            // Enough data?
            if available_data < node.data_cost {
                return false;
            }
            true
        } else {
            false
        }
    }

    /// Check if a node can be selected for research (ignores current data)
    pub fn can_select(&self, id: &str, unlocked: &[String]) -> bool {
        if let Some(node) = self.get_node(id) {
            if unlocked.contains(&node.id) {
                return false;
            }
            if !node.prerequisites.iter().all(|p| unlocked.contains(p)) {
                return false;
            }
            true
        } else {
            false
        }
    }

    /// Get all nodes that are available for research
    pub fn available_research(&self, unlocked: &[String], available_data: f32) -> Vec<&ResearchNode> {
        self.nodes
            .iter()
            .filter(|n| self.can_research(&n.id, unlocked, available_data))
            .collect()
    }

    /// Get connections for visualization (from -> to)
    pub fn get_connections(&self) -> Vec<(&ResearchNode, &ResearchNode)> {
        let mut connections = Vec::new();
        for node in &self.nodes {
            for prereq_id in &node.prerequisites {
                if let Some(prereq) = self.get_node(prereq_id) {
                    connections.push((prereq, node));
                }
            }
        }
        connections
    }
}

/// Research state for a player
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchState {
    pub unlocked: Vec<String>,
    pub current_research: Option<String>,
    pub research_progress: f32,
}

impl Default for ResearchState {
    fn default() -> Self {
        Self {
            unlocked: vec!["core".to_string(), "basic_mining".to_string()],
            current_research: None,
            research_progress: 0.0,
        }
    }
}

impl ResearchState {
    /// Start researching a tech
    pub fn start_research(&mut self, tech_id: &str, tree: &ResearchTree, _available_data: f32) -> bool {
        if tree.can_select(tech_id, &self.unlocked) {
            self.current_research = Some(tech_id.to_string());
            self.research_progress = 0.0;
            true
        } else {
            false
        }
    }

    /// Complete current research
    pub fn complete_research(&mut self) -> Option<String> {
        if let Some(tech) = self.current_research.take() {
            self.unlocked.push(tech.clone());
            self.research_progress = 0.0;
            Some(tech)
        } else {
            None
        }
    }

    /// Check if a tech is unlocked
    pub fn is_unlocked(&self, tech_id: &str) -> bool {
        self.unlocked.contains(&tech_id.to_string())
    }
}
