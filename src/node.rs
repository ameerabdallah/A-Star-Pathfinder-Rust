use std::marker::PhantomData;

#[derive(Debug)]
pub(crate) enum NodeState {
    Wall,
    Start,
    Stop,
    Open,
}

#[derive(Debug)]
pub(crate) struct Pos {
    pub x: i8,
    pub y: i8,
}


#[derive(Debug)]
pub(crate) struct Node<'a> {
    pub position: Pos,
    parent: Option<&'a Node<'a>>,
    node_type: NodeState,
    pub g: f64,
    pub h: f64,
    pub f: f64,
}

impl<'a> Node<'a> {
    pub(crate) fn new() -> Node<'a> {
        Node {
            position: (Pos { x: 0, y: 0 }),
            parent: Option::None,
            node_type: NodeState::Open,
            g: (0.0),
            h: (0.0),
            f: (0.0),
        }
    }
    pub(crate) fn set_node_type(&mut self, node_type: NodeState) {
        self.node_type = node_type;
    }

    pub(crate) fn calculate_f_cost(&mut self) {
        self.f = self.h + self.h;
    }
}
