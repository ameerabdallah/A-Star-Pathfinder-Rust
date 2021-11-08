use crate::pathfinder::Node;
use crate::pathfinder::NodeType;

pub mod pathfinder {

    const GRID_SIZE_X: usize = 50;
    const GRID_SIZE_Y: usize = 50;
    
    struct Grid {
        array: [[Node;GRID_SIZE_X]; GRID_SIZE_Y],
        start_node: Node,
        end_node: Node,
    }
    
    impl Grid {
        pub fn set_start_node(&mut self, node: Node) {
            self.start_node.set_node_type(NodeType::Empty);
            self.start_node = node;
        }

        pub fn set_end_node(&mut self, node: Node) {
            self.end_node = node;
        }
    }
}

