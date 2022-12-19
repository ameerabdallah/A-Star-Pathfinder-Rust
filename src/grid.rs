use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::vec;

use colored::Colorize;
use colored::ColoredString;

use crate::node::Node;
use crate::node::NodeState;
use crate::node::Pos;
use crate::node;
use rand::prelude::*;
use priority_queue::PriorityQueue;


#[derive(Debug)]

pub(crate) struct Grid {
    map: Vec<Vec<Node>>,
    start_pos: Pos,
    end_pos: Pos,
    x_size: usize,
    y_size: usize
}

impl Grid{

    pub(crate) fn generate_random_grid(x_size: usize, y_size:usize) -> Grid{
        let mut rng = rand::thread_rng();
        let mut map: Vec<Vec<Node>> = Vec::new();
        for i in 0..x_size {
            let mut row: Vec<Node> = Vec::new();
            for j in 0..y_size {
                let mut node = Node::new();
                node.pos = Pos{x:i, y:j};
                if rng.gen::<f64>() > 0.3 {
                    node.node_state = NodeState::Open;
                }
                else
                {
                    node.node_state = NodeState::Wall;
                }
                row.push(node);
            }
            map.push(row);
        }

        let start_pos = Pos {x: rng.gen_range(0..x_size), y: rng.gen_range(0..y_size)};
        let end_pos = Pos {x: rng.gen_range(0..x_size), y: rng.gen_range(0..y_size)};

        map[start_pos.x][start_pos.y].node_state = NodeState::Start;
        map[end_pos.x][end_pos.y].node_state = NodeState::Destination;

        let mut grid = Grid { map, start_pos, end_pos, x_size, y_size };
        grid.calculate_h_costs();
        grid

    }

    pub(crate) fn from(map_vec: Vec<String>, x_size: usize, y_size: usize) -> Grid {
        
        let mut map: Vec<Vec<Node>> = vec![vec![Node::new(); x_size];y_size];
        let mut start_pos= Pos{x:0, y:0};
        let mut end_pos= Pos{x:0, y:0};
        
        for (i, line) in map_vec.iter().enumerate(){
            for (j, next_char) in line.chars().enumerate(){

                match next_char{
                    node::WALL  => map[i][j].node_state = NodeState::Wall,
                    node::DESTINATION   => {
                        map[i][j].node_state = NodeState::Destination;
                        end_pos = Pos {x:i, y:j};
                    },
                    node::START => {
                        map[i][j].node_state = NodeState::Start;
                        start_pos = Pos {x:i, y:j};
                    },
                    node::OPEN   => map[i][j].node_state = NodeState::Open,
                    _y => {
                        map[i][j].node_state = NodeState::Open
                    }
                }
                map[i][j].pos = Pos{x:i, y:j};
            }
        }
        let mut grid = Grid { map, start_pos, end_pos, x_size, y_size };
        grid.calculate_h_costs();
        grid
    }

    

    // call once at the beginning of a-star to set up the h cost values
    fn calculate_h_costs(&mut self) {
        for row in &mut self.map {
            for node in row {
                node.h = dist(&self.end_pos, &node.pos);
            }
        }
    }

    pub(crate) fn run_a_star(&mut self, print_result: bool) -> Vec<Box<Node>> {
        let mut backtrack_vec = Vec::new();
        let mut open_set: PriorityQueue<Node, F> = PriorityQueue::new();

        let curr_f = self.map[self.start_pos.x][self.start_pos.y].calculate_f_cost();
        self.map[self.start_pos.x][self.start_pos.y].g = 0.0;
        open_set.push(self.map[self.start_pos.x][self.start_pos.y].clone(), F{f:curr_f});

        loop {
            let curr_node;
            match open_set.pop() {
                Some(i) => 
                {
                    curr_node = i.0;
                },
                None => {dbg!("no more elements");break;},
            };
            
            if self.is_goal(&curr_node) {
                backtrack_vec = self.backtrack(Box::new(curr_node));
                break;
            }
            
            // search surrounding neighbors
            for x in -1..=1 {
                for y in -1..=1 {
                    // to skip the curr_node
                    let i = x + curr_node.pos.x as isize;
                    let j = y + curr_node.pos.y as isize;
                    if i as usize == curr_node.pos.x && j as usize == curr_node.pos.y { continue; } 
                    
                    if i >= 0 && i < self.x_size as isize && j >= 0 && j < self.y_size as isize {

                        let mut neighbor = self.map[i as usize][j as usize].clone();
                        if let NodeState::Wall = neighbor.node_state { continue; } 
                        let temp_g = curr_node.g + dist(&neighbor.pos, &curr_node.pos);
                        
                        if temp_g < neighbor.g {
                            neighbor.g = temp_g;
                            self.map[i as usize][j as usize].g = temp_g;
                            neighbor.parent = Some(Box::new(curr_node.clone()));
                            let curr_f = neighbor.calculate_f_cost();
                            open_set.push(neighbor, F{f:curr_f});
                        }
                    }
                }
            }
        }


        if print_result {
            self.print_path(&backtrack_vec);
        }
        backtrack_vec
    }

    fn is_goal(&self, other: &Node) -> bool {
        self.end_pos == other.pos
    }
    pub(crate) fn print_grid(&self) {
        
        for _i in 0..self.y_size+3 {
            print!("{}", "▓▓".black().on_truecolor(215, 135, 0))
        }
        println!();
        io::stdout().flush().unwrap();
        for row in &self.map {
            let mut c: ColoredString;
            print!("{}", "▓▓▓".black().on_truecolor(215, 135, 0));
            for node in row {
                match node.node_state{
                    NodeState::Wall => c = "██".black(),
                    NodeState::Destination => c = "██".red(),
                    NodeState::Open => c = "██".bright_white(),
                    NodeState::Start => c = "██".blue(),
                }
                print!("{}", c);
            }
            print!("{}", "▓▓▓".black().on_truecolor(215, 135, 0));
            println!();
            io::stdout().flush().unwrap();
        }
        for _i in 0..self.y_size+3 {
            print!("{}", "▓▓".black().on_truecolor(215, 135, 0))
        }
        println!();
        io::stdout().flush().unwrap();
    }

    fn print_path(&self, path: &Vec<Box<Node>>) {

        for _i in 0..self.y_size+3 {
            print!("{}", "▓▓".black().on_truecolor(215, 135, 0))
        }
        println!();
        io::stdout().flush().unwrap();

        for row in &self.map {
            let mut c: ColoredString;
            print!("{}", "▓▓▓".black().on_truecolor(215, 135, 0));
            for node in row {
                match node.node_state{
                    NodeState::Wall => c = "██".black(),
                    NodeState::Destination => c = "██".red(),
                    NodeState::Open => {
                        if path.iter().any(|i| i.pos == node.pos) {
                            c = "██".green();
                        }
                        else {
                            c = "██".bright_white();
                        }
                    },
                    NodeState::Start => c = "██".blue(),
                }
                print!("{}", c);
            }
            print!("{}", "▓▓▓".black().on_truecolor(215, 135, 0));
            println!();
            io::stdout().flush().unwrap();
        }
        for _i in 0..self.y_size+3 {
            print!("{}", "▓▓".black().on_truecolor(215, 135, 0))
        }
        println!();
        io::stdout().flush().unwrap();

    }

    fn backtrack(&self, last_node: Box<Node>) -> Vec<Box<Node>> {
        let mut next_node = last_node;
        let mut vec = Vec::new();

            
        loop {
            match next_node.parent {
                None => break,
                Some(node) =>{
                    next_node = node;
                    vec.push(next_node.clone());
                }
            }
        }

        for element in &mut vec {
            element.parent = Option::None;
        }

        vec
    }
}

fn dist(p: &Pos, q: &Pos) -> f64 {
    let diff_x = p.x as isize - q.x as isize;
    let diff_y = p.y as isize - q.y as isize;
    let d = (diff_x.pow(2) + diff_y.pow(2)) as f64;
    
    d.sqrt()
}

#[derive(Debug)]
struct F {
    f: f64
}

impl Ord for F {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.f > other.f { std::cmp::Ordering::Less }
        else if self.f < other.f {std::cmp::Ordering::Greater }
        else { std::cmp::Ordering::Equal }

    }
}

impl PartialOrd for F {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for F {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl Eq for F {

}