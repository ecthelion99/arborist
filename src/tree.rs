use glam::Vec3;
use slotmap::{SlotMap, new_key_type};

new_key_type! { pub struct GraphNodeId;}

pub struct TreeGraphNode<T> {
    parent: Option<GraphNodeId>,
    children: Vec<GraphNodeId>,
    payload: T,
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

    pub fn insert_node(&mut self, parent_id: GraphNodeId, payload: T) -> GraphNodeId {
        let node: TreeGraphNode<T> = TreeGraphNode {
            parent: Some(parent_id),
            children: Vec::new(),
            payload,
        };
        self.nodes_sm.insert(node)
    }
}

pub struct TreePayload {
    translation: Vec3,
}

impl TreePayload {
    pub fn new(translation: Vec3) -> TreePayload {
        TreePayload { translation }
    }
}
