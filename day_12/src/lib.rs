use std::cell::{Ref, RefCell};
use std::fs;
use std::collections::{HashMap, VecDeque};
use std::io;
use std::io::BufRead;
use std::path::Path;

struct Node {
    x: i32,
    y: i32,
    elevation: i32,
    cost: u32,
    explored: bool,
    is_end: bool
}

impl Node {
    fn set_explored(&mut self) {
        self.explored = true;
    }
    fn set_cost(&mut self, cost: u32) {
        self.cost = cost;
    }
}

fn is_end(node: &RefCell<Node>) -> bool {
    node.borrow().is_end
}

fn is_a(node: &RefCell<Node>) -> bool {
    node.borrow().elevation == 1
}

fn can_step_up(node_elevation: i32, neighbour_elevation: i32) -> bool {
    neighbour_elevation <= node_elevation + 1
}

fn can_step_down(node_elevation: i32, neighbour_elevation: i32) -> bool {
     node_elevation <= neighbour_elevation + 1
}

struct NodeSearch {
    map: HashMap<(i32, i32), RefCell<Node>>,
    stop: fn (&RefCell<Node>) -> bool,
    can_step_on_neighbour: fn (i32, i32) -> bool
}

impl NodeSearch {
    fn new(map: HashMap<(i32, i32), RefCell<Node>>) -> Self {
        NodeSearch {
            map,
            stop: is_end,
            can_step_on_neighbour: can_step_up
        }
    }

    fn bfs(&mut self, point: (i32, i32)) -> Option<u32> {
        let mut queue= VecDeque::new();
        let start_node = self.map.get(&point).unwrap();
        start_node.borrow_mut().set_explored();
        queue.push_front(start_node);
        while !queue.is_empty() {
            let node = queue.pop_back().unwrap();
            if (self.stop)(node) {
                return Some(node.borrow().cost)
            }
            let unvisited_neighbours = self.get_unvisited_neighbours(&node.borrow());
            for neighbour_node in unvisited_neighbours {
                neighbour_node.borrow_mut().set_explored();
                neighbour_node.borrow_mut().set_cost(node.borrow().cost + 1);
                queue.push_front(neighbour_node);
            }
        }
        None
    }

    fn get_unvisited_neighbours(&self, node: &Ref<Node>) -> Vec<&RefCell<Node>> {
        let mut neighbour_vec = vec![];
        let neighbour_points = vec![
            (node.x - 1, node.y),
            (node.x + 1, node.y),
            (node.x, node.y - 1),
            (node.x, node.y + 1)
        ];
        for neighbour_point in neighbour_points {
            if self.is_unvisited_neighbour(&node, &neighbour_point){
                neighbour_vec.push(self.map.get(&neighbour_point).unwrap());
            }
        }
        neighbour_vec
    }

    fn is_unvisited_neighbour(&self, node: &Ref<Node>, neighbour_point: &(i32, i32)) -> bool{
        match self.map.get(neighbour_point) {
            Some(neighbour_node) => {
                !neighbour_node.borrow().explored && (self.can_step_on_neighbour)(node.elevation, neighbour_node.borrow().elevation)
            },
            None => false
        }
    }
}

fn read_in_file_system(file_path: &str) -> Result<(HashMap<(i32, i32), RefCell<Node>>, (i32, i32), (i32, i32)), &str> {
    if let Ok(lines) = read_lines(file_path) {
        let mut map = HashMap::new();
        let mut row = 0;
        let mut start_point = None;
        let mut end_point = None;
        for line in lines {
            if let Ok(l) = line {
                let mut column = 0;
                for c in l.chars() {
                    let mut is_end = false;
                    if c == 'E' {
                        is_end = true;
                        end_point = Some((column, row));
                    }
                    map.insert((column, row), RefCell::new(Node {
                        x: column,
                        y: row,
                        elevation: map_to_elevation(&c).unwrap(),
                        cost: 0,
                        explored: false,
                        is_end
                    }));
                    if c == 'S' {
                        start_point = Some((column, row));
                    }
                    column += 1;
                }
                row += 1;
            }
        }
        Ok((map, start_point.unwrap(), end_point.unwrap()))
    } else {
        Err("no valid file")
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn map_to_elevation(c: &char) -> Option<i32> {
    match c {
        'a' => Some(1),
        'b' => Some(2),
        'c' => Some(3),
        'd' => Some(4),
        'e' => Some(5),
        'f' => Some(6),
        'g' => Some(7),
        'h' => Some(8),
        'i' => Some(9),
        'j' => Some(10),
        'k' => Some(11),
        'l' => Some(12),
        'm' => Some(13),
        'n' => Some(14),
        'o' => Some(15),
        'p' => Some(16),
        'q' => Some(17),
        'r' => Some(18),
        's' => Some(19),
        't' => Some(20),
        'u' => Some(21),
        'v' => Some(22),
        'w' => Some(23),
        'x' => Some(24),
        'y' => Some(25),
        'z' => Some(26),
        'S' => Some(1),
        'E' => Some(26),
        _ => None
    }
}

pub fn solve_routine(file_path: &str) -> Option<u32> {
    let (map, start_point, _) = read_in_file_system(file_path).unwrap();
    let mut node_search = NodeSearch::new(map);
    node_search.bfs(start_point)
}

pub fn solve_routine_2(file_path: &str) -> Option<u32> {
    let (map, _, end_point) = read_in_file_system(file_path).unwrap();
    let mut node_search = NodeSearch::new(map);
    node_search.stop = is_a;
    node_search.can_step_on_neighbour = can_step_down;
    node_search.bfs(end_point)
}

#[cfg(test)]
mod tests {
    use crate::{solve_routine, solve_routine_2};

    #[test]
    fn test_puzzle_1() {
        assert_eq!(solve_routine("input_test").unwrap(), 31)
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(solve_routine_2("input_test").unwrap(), 29)
    }

}