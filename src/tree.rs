use glam::Vec3;
use slotmap::{SlotMap, new_key_type};

new_key_type! { pub struct GraphNodeId;}

pub struct TreeGraphNode<T> {
    pub parent: Option<GraphNodeId>,
    pub children: Vec<GraphNodeId>,
    pub payload: T,
}

pub struct TreeGraph<T> {
    nodes_sm: SlotMap<GraphNodeId, TreeGraphNode<T>>,
    pub root_id: GraphNodeId,
}

impl<T> TreeGraph<T> {
    pub fn new(root_payload: T) -> TreeGraph<T> {
        let mut nodes_sm: SlotMap<GraphNodeId, TreeGraphNode<T>> = SlotMap::with_key();
        let root_node: TreeGraphNode<T> = TreeGraphNode {
            parent: None,
            children: Vec::new(),
            payload: root_payload,
        };
        let root_id = nodes_sm.insert(root_node);

        TreeGraph { nodes_sm, root_id }
    }

    pub fn get_node_by_id(&self, id: GraphNodeId) -> Option<&TreeGraphNode<T>> {
        self.nodes_sm.get(id)
    }

    pub fn get_node_by_id_mut(&mut self, id: GraphNodeId) -> Option<&mut TreeGraphNode<T>> {
        self.nodes_sm.get_mut(id)
    }

    pub fn get_root_node(&self) -> &TreeGraphNode<T> {
        &self.nodes_sm[self.root_id]
    }

    pub fn insert_node(&mut self, parent_id: GraphNodeId, payload: T) -> Option<GraphNodeId> {
        if !self.nodes_sm.contains_key(parent_id) {
            return None;
        }
        let node: TreeGraphNode<T> = TreeGraphNode {
            parent: Some(parent_id),
            children: Vec::new(),
            payload,
        };
        let child_id = self.nodes_sm.insert(node);
        self.get_node_by_id_mut(parent_id)
            .unwrap()
            .children
            .push(child_id);
        Some(child_id)
    }
}

pub struct TreePayload {
    pub translation: Vec3,
}

impl TreePayload {
    pub fn new(translation: Vec3) -> TreePayload {
        TreePayload { translation }
    }
}
