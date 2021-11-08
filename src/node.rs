pub mod pathfinder {
    enum NodeType {
        Wall,
        Start,
        Stop,
        Open,
    }
    
    struct Node {
        x: u8,
        y: u8,
        n_type: NodeType::Open,
    }
    
    impl Node {
        
    }
}

