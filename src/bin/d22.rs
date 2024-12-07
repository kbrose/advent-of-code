use std::fs;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: u64,
    y: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Range {
    lo: u64,
    range: u64,
}

impl Range {
    fn hi(&self) -> u64 {
        self.lo + self.range
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Brick {
    start: Coord,
    end: Coord,
    height_range: Range,
}

impl Brick {
    fn x_lo(&self) -> u64 {
        std::cmp::min(self.start.x, self.end.x)
    }
    fn x_hi(&self) -> u64 {
        std::cmp::max(self.start.x, self.end.x)
    }
    fn y_lo(&self) -> u64 {
        std::cmp::min(self.start.y, self.end.y)
    }
    fn y_hi(&self) -> u64 {
        std::cmp::max(self.start.y, self.end.y)
    }
    fn z_lo(&self) -> u64 {
        self.height_range.lo
    }
    fn z_hi(&self) -> u64 {
        self.height_range.hi()
    }

    fn overlaps(&self, other: &Self) -> bool {
        if (self.x_lo() > other.x_hi())
            || (self.x_hi() < other.x_lo())
            || (self.y_lo() > other.y_hi())
            || (self.y_hi() < other.y_lo())
        {
            false
        } else {
            // I thought there would be more to it, but actually that's sufficient!
            true
        }
    }
}

mod private {
    use super::{Brick, Range};
    use std::cell::RefCell;
    use std::collections::HashSet;
    use std::marker::PhantomData;
    use std::rc::Rc;

    type RcBrick = Rc<RefCell<Brick>>;

    #[derive(Debug)]
    struct Node {
        brick: RcBrick,
        // Keep track of the edges in both directions
        bricks_below: Vec<RcBrick>,
        bricks_above: Vec<RcBrick>,
        support_count: usize,
    }

    type RcNode = Rc<RefCell<Node>>;

    #[derive(Debug)]
    pub struct Bricks {
        bricks: Vec<RcNode>,
        _phantom: PhantomData<()>, // zero-runtime-cost private member to prevent bare instantiation
    }

    pub fn new_bricks_graph(mut bricks: Vec<Brick>) -> Bricks {
        // Sort by z so bricks that are lower are come earlier
        bricks.sort_by(|a, b| a.z_lo().cmp(&b.z_lo()));
        let bricks: Vec<RcBrick> = bricks
            .into_iter()
            .map(|b| Rc::new(RefCell::new(b)))
            .collect();
        // When processing gravity, we care about the bricks below it.
        // But in other cases, we care about the bricks above it. Track both.
        let mut downward_overlaps: Vec<Vec<RcBrick>> = Vec::new();
        let mut upward_overlaps: Vec<Vec<RcBrick>> = Vec::new();
        for brick in &bricks {
            let mut downward_bricks: Vec<RcBrick> = Vec::new();
            let mut upward_bricks: Vec<RcBrick> = Vec::new();
            for brick2 in &bricks {
                if brick.borrow().overlaps(&brick2.borrow()) {
                    if brick2.borrow().z_lo() < brick.borrow().z_lo() {
                        downward_bricks.push(Rc::clone(&brick2));
                    } else {
                        upward_bricks.push(Rc::clone(&brick2));
                    }
                }
            }
            downward_overlaps.push(downward_bricks);
            upward_overlaps.push(upward_bricks);
        }

        // Process gravity
        for (brick, downward_overlaps) in bricks.iter().zip(downward_overlaps.iter()) {
            let mut new_starting_height = 1;
            for overlapper in downward_overlaps {
                new_starting_height = std::cmp::max(
                    new_starting_height,
                    overlapper.borrow().height_range.hi() + 1,
                );
            }
            let range = brick.borrow().height_range.range;
            brick.borrow_mut().height_range = Range {
                lo: new_starting_height,
                range,
            }
        }

        // Find edges (i.e. supporting bricks)
        // Each brick will point to the bricks that it supports
        let nodes: Vec<RcNode> = bricks
            .into_iter()
            .zip(downward_overlaps.iter())
            .zip(upward_overlaps.iter())
            .map(|((brick, downward_overlaps), upward_overlaps)| {
                let brick_height_bottom = brick.borrow().z_lo();
                let bricks_below: Vec<Rc<RefCell<Brick>>> = downward_overlaps
                    .iter()
                    .filter(|overlapper| overlapper.borrow().z_hi() + 1 == brick_height_bottom)
                    .map(|x| Rc::clone(&x))
                    .collect();
                let brick_height_top = brick.borrow().z_hi();
                let bricks_above: Vec<Rc<RefCell<Brick>>> = upward_overlaps
                    .iter()
                    .filter(|overlapper| overlapper.borrow().z_lo() == brick_height_top + 1)
                    .map(|x| Rc::clone(&x))
                    .collect();
                Rc::new(RefCell::new(Node {
                    support_count: bricks_below.len(),
                    brick,
                    bricks_below,
                    bricks_above,
                }))
            })
            .collect();

        Bricks {
            bricks: nodes,
            _phantom: PhantomData,
        }
    }

    impl Bricks {
        pub fn count_disintegrable(&self) -> usize {
            let mut load_bearing_bricks: HashSet<Brick> = HashSet::new();
            for node in &self.bricks {
                if node.borrow().bricks_below.len() == 1 {
                    load_bearing_bricks.insert(*node.borrow().bricks_below[0].borrow());
                }
            }
            self.bricks.len() - load_bearing_bricks.len()
        }

        fn reset_support_counts(&mut self) -> () {
            for node in &mut self.bricks {
                let mut node = node.borrow_mut();
                node.support_count = node.bricks_below.len();
            }
        }

        fn node_at_brick(&self, brick: &RcBrick) -> Option<&RcNode> {
            for node in &self.bricks {
                if node.borrow().brick == *brick {
                    return Some(&node);
                }
            }
            None
        }

        pub fn count_total_falls(&mut self) -> usize {
            let mut falls: usize = 0;
            for i in 0..self.bricks.len() {
                // Traverse the graph breadth first. That's important because the graph
                // nodes are stored from bottom to top, and we want to process them
                // in that order.
                let mut nodes_to_process: Vec<&RcNode> = vec![&self.bricks[i]];
                while nodes_to_process.len() > 0 {
                    let node = nodes_to_process.pop().unwrap();
                    for supported_brick in &node.borrow().bricks_above {
                        let mut supported_node =
                            self.node_at_brick(supported_brick).unwrap().borrow_mut();
                        supported_node.support_count =
                            supported_node.support_count.checked_sub(1).unwrap();
                        if supported_node.support_count == 0 {
                            std::mem::drop(supported_node);
                            nodes_to_process.push(self.node_at_brick(supported_brick).unwrap());
                        }
                    }
                }
                let x = self
                    .bricks
                    .iter()
                    .filter(|node| {
                        (node.borrow().support_count == 0)
                            && (node.borrow().brick.borrow().z_lo() > 1)
                    })
                    .count();
                falls += x;

                self.reset_support_counts();
            }
            falls
        }
    }
}

use private::{new_bricks_graph, Bricks};

fn parse_brick_str(brick_str: &str) -> Brick {
    let coords: Vec<Vec<u64>> = brick_str
        .split('~')
        .map(|s| s.split(',').map(|c| c.parse().unwrap()).collect())
        .collect();
    // We expect two sets of 3 coords each
    assert_eq!(coords.len(), 2);
    for coord in &coords {
        assert_eq!(coord.len(), 3);
    }
    let start = Coord {
        x: coords[0][0],
        y: coords[0][1],
    };
    let end = Coord {
        x: coords[1][0],
        y: coords[1][1],
    };
    let height_lo = std::cmp::min(coords[0][2], coords[1][2]);
    let height_hi = std::cmp::max(coords[0][2], coords[1][2]);
    let height_range = Range {
        lo: height_lo,
        range: height_hi - height_lo,
    };
    Brick {
        start,
        end,
        height_range,
    }
}

fn parse_input(contents: &String) -> Bricks {
    new_bricks_graph(contents.trim().split('\n').map(parse_brick_str).collect())
}

fn compute_1(contents: &String) -> usize {
    let bricks = parse_input(contents);
    bricks.count_disintegrable()
}

fn compute_2(contents: &String) -> usize {
    let mut bricks = parse_input(contents);
    bricks.count_total_falls()
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d22.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(413, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(41610, result);
    println!("part 2: {result}");
}
