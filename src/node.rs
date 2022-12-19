use std::hash::Hash;

pub(crate) const WALL: char =  '█';
pub(crate) const OPEN: char =  ' ';
pub(crate) const START: char =  'O';
pub(crate) const DESTINATION: char =   'X';

#[derive(Debug, Copy, Clone)]
pub(crate) enum NodeState {
    Wall,
    Start,
    Destination,
    Open,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) struct Pos {
    pub x: usize,
    pub y: usize,
}


#[derive(Debug, Clone)]
pub(crate) struct Node {
    pub pos: Pos,
    pub parent: Option<Box<Node>>,
    pub node_state: NodeState,
    pub g: f64,
    pub h: f64,
    pub f: f64,
}

impl Node {
    pub(crate) fn new() -> Node {
        Node {
            pos: (Pos { x: 0, y: 0 }),
            parent: Option::None,
            node_state: NodeState::Open,
            g: (f64::MAX),
            h: (f64::MAX),
            f: (f64::MAX),
        }
    }
    pub(crate) fn calculate_f_cost(&mut self) -> f64 {
        self.f = self.h + self.g;
        self.f
    }
}




impl<'a> Eq for Node {}

impl<'a> PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos.x == other.pos.x && self.pos.y == other.pos.y
    }
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut hash = 17;
        hash = ((hash+self.pos.x) << 5) - (hash+self.pos.x);
        hash = ((hash+self.pos.y) << 5) - (hash+self.pos.y);

        state.write_usize(hash);
    }
}