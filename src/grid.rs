use crate::node::Node;
use crate::node::NodeState;
use crate::node::Pos;

const GRID_SIZE_X: i8 = 10;
const GRID_SIZE_Y: i8 = 10;

#[derive(Debug)]

struct Grid <'a>{
    array: [[Node<'a>;GRID_SIZE_X as usize]; GRID_SIZE_Y as usize],
    start_pos: Pos,
    end_pos: Pos,
}

impl<'a> Grid<'a> {
    pub(crate) fn new() -> Grid<'a> {
        let array: [[Node<'a>;GRID_SIZE_X  as usize]; GRID_SIZE_Y as usize] = [
            [Node::new();GRID_SIZE_X as usize];GRID_SIZE_Y as usize
            ];
        Grid { 
            array: array, 
            start_pos: Pos {x: 0, y: 0}, 
            end_pos: Pos {x: GRID_SIZE_X-1,y: GRID_SIZE_Y-1}
        }
    }

    fn set_start_node(&mut self, new_pos: &Pos) {

        // make sure old start_node is set back to open
        self.array[self.start_pos.x as usize][self.start_pos.y as usize].set_node_type(NodeState::Open);
        
        // set the new start_node to wall
        self.start_pos = Pos {x: new_pos.x, y: new_pos.y};
        self.array[self.start_pos.x as usize][self.start_pos.y as usize].set_node_type(NodeState::Open);
    }

    fn set_end_node(&mut self, node: &'a mut Node<'a>) {
        .set_node_type(NodeState::Open);

        node.set_node_type(NodeState::Start);

        self.end_pos = node;
    }

    fn get_start_node(&self) -> &Node<'a> {
        &self.start_pos
    }

    fn get_end_node(&self) -> &Node<'a> {
        &self.end_pos
    }

    // call once at the beginning of a-star to set up the h cost values
    fn calculate_h_costs(&mut self) {
        for row in &mut self.array {
            for node in row {
                node.h = (
                    ((self.end_pos.position.x - node.position.x).pow(2) +
                     (self.end_pos.position.y - node.position.y).pow(2)) as f64).sqrt();
            }
        }
    }

    fn a_star_step(&mut self, curr_node: &Node) {
        for row in &mut self.array {
            for node in row {

            }
        }
    }


}



