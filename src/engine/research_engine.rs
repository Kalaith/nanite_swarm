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
    pub position: (f32, f32), // Visual position in neural network
}

impl ResearchNode {
    pub fn new(
        id: &str,
        name: &str,
        description: &str,
        cost: f32,
        prereqs: Vec<&str>,
        pos: (f32, f32),
    ) -> Self {
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
        crate::data::game_data().research_tree()
    }
}

impl ResearchTree {
    pub fn from_nodes(nodes: Vec<ResearchNode>) -> Self {
        Self { nodes }
    }

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
    pub fn available_research(
        &self,
        unlocked: &[String],
        available_data: f32,
    ) -> Vec<&ResearchNode> {
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
            unlocked: crate::data::game_data().research.starting_unlocked.clone(),
            current_research: None,
            research_progress: 0.0,
        }
    }
}

impl ResearchState {
    /// Start researching a tech
    pub fn start_research(
        &mut self,
        tech_id: &str,
        tree: &ResearchTree,
        _available_data: f32,
    ) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn tree() -> ResearchTree {
        ResearchTree::from_nodes(vec![
            ResearchNode::new("a", "A", "first", 10.0, vec![], (0.0, 0.0)),
            ResearchNode::new("b", "B", "needs a", 20.0, vec!["a"], (1.0, 0.0)),
            ResearchNode::new("c", "C", "needs a and b", 30.0, vec!["a", "b"], (2.0, 0.0)),
        ])
    }

    #[test]
    fn can_research_requires_prerequisites_and_data() {
        let tree = tree();
        let unlocked = vec![];
        assert!(tree.can_research("a", &unlocked, 10.0));
        assert!(!tree.can_research("b", &unlocked, 100.0)); // prereq "a" missing
        assert!(!tree.can_research("a", &unlocked, 5.0)); // not enough data
    }

    #[test]
    fn can_research_rejects_already_unlocked() {
        let tree = tree();
        let unlocked = vec!["a".to_string()];
        assert!(!tree.can_research("a", &unlocked, 100.0));
    }

    #[test]
    fn can_select_ignores_available_data() {
        let tree = tree();
        let unlocked = vec!["a".to_string()];
        assert!(tree.can_select("b", &unlocked));
        assert!(!tree.can_select("c", &unlocked)); // still needs "b"
    }

    #[test]
    fn available_research_filters_by_prereqs_and_data() {
        let tree = tree();
        let unlocked = vec!["a".to_string()];
        let available = tree.available_research(&unlocked, 20.0);
        assert_eq!(available.len(), 1);
        assert_eq!(available[0].id, "b");
    }

    #[test]
    fn get_connections_pairs_each_node_with_its_prerequisites() {
        let tree = tree();
        let connections = tree.get_connections();
        // "a" has no prereqs, "b" contributes one edge (a->b), "c" contributes two (a->c, b->c).
        assert_eq!(connections.len(), 3);
        assert!(connections
            .iter()
            .any(|(from, to)| from.id == "a" && to.id == "b"));
        assert!(connections
            .iter()
            .any(|(from, to)| from.id == "b" && to.id == "c"));
    }

    #[test]
    fn start_research_requires_selectable_tech() {
        let tree = tree();
        let mut state = ResearchState {
            unlocked: vec![],
            current_research: None,
            research_progress: 5.0,
        };

        assert!(!state.start_research("b", &tree, 100.0)); // prereq missing
        assert!(state.current_research.is_none());

        assert!(state.start_research("a", &tree, 100.0));
        assert_eq!(state.current_research, Some("a".to_string()));
        assert_eq!(state.research_progress, 0.0);
    }

    #[test]
    fn complete_research_unlocks_tech_and_clears_progress() {
        let mut state = ResearchState {
            unlocked: vec![],
            current_research: Some("a".to_string()),
            research_progress: 10.0,
        };

        let completed = state.complete_research();
        assert_eq!(completed, Some("a".to_string()));
        assert!(state.is_unlocked("a"));
        assert_eq!(state.research_progress, 0.0);
        assert!(state.current_research.is_none());
    }

    #[test]
    fn complete_research_is_noop_with_nothing_in_progress() {
        let mut state = ResearchState {
            unlocked: vec![],
            current_research: None,
            research_progress: 0.0,
        };
        assert_eq!(state.complete_research(), None);
    }
}
