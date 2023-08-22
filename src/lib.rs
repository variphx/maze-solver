use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(PartialEq, Eq)]
pub enum Tile {
    Wall,
    Path,
}

pub struct MazeSolver {
    maze: Maze,
    starting_point: StartingPoint,
    ending_point: EndingPoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordination {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq)]
struct ManhattanDistance;

#[derive(PartialEq, Eq)]
struct Agent {
    coordination: Coordination,
    steps: usize,
    priority_points: usize,
}

pub type Maze = Vec<Vec<Tile>>;
pub type StartingPoint = Coordination;
pub type EndingPoint = Coordination;

type Frontier = BinaryHeap<Agent>;
type ExploredSet = HashSet<Coordination>;
type ParentMap = HashMap<Coordination, Coordination>;

impl MazeSolver {
    pub fn new(maze: Maze, starting_point: StartingPoint, ending_point: EndingPoint) -> Self {
        Self {
            maze,
            starting_point,
            ending_point,
        }
    }

    #[inline]
    fn is_solved(&self, agent: &Agent) -> bool {
        self.ending_point == agent.coordination
    }

    pub fn solution(&self) -> Result<Vec<Coordination>, &str> {
        let mut frontier = Frontier::new();
        let mut explored_set = ExploredSet::new();
        let mut parent_map = ParentMap::new();
        let mut final_agent = None;

        let initial_agent = Agent {
            coordination: self.starting_point,
            steps: 0,
            priority_points: ManhattanDistance::distance(&self.starting_point, &self.ending_point),
        };

        frontier.push(initial_agent);

        while !frontier.is_empty() {
            let agent = frontier.pop().unwrap();
            let _ = explored_set.insert(agent.coordination);

            if self.is_solved(&agent) {
                final_agent = Some(agent);
                break;
            }

            for neighbor in agent.neighbors_in(&self.maze) {
                if !explored_set.contains(&neighbor) && neighbor.is_movable_in(&self.maze) {
                    let new_steps = agent.steps + 1;
                    frontier.push(Agent {
                        coordination: neighbor,
                        steps: new_steps,
                        priority_points: new_steps
                            + ManhattanDistance::distance(&neighbor, &self.ending_point),
                    });
                    let _ = parent_map.insert(neighbor, agent.coordination);
                }
            }
        }

        if let Some(agent) = final_agent {
            let mut result = vec![agent.coordination];
            while let Some(&parent) = parent_map.get(result.last().unwrap()) {
                result.push(parent);
            }
            result.reverse();
            Ok(result)
        } else {
            Err("No solution")
        }
    }
}

impl Coordination {
    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    #[inline]
    fn absolute_manhattan(&self) -> usize {
        self.x + self.y
    }

    #[inline]
    fn is_movable_in(&self, maze: &Maze) -> bool {
        maze[self.y][self.x] == Tile::Path
    }
}

impl ManhattanDistance {
    #[inline]
    fn distance(from: &Coordination, to: &Coordination) -> usize {
        from.absolute_manhattan().abs_diff(to.absolute_manhattan())
    }
}

impl Agent {
    #[inline]
    fn neighbors_in(&self, maze: &Maze) -> Vec<Coordination> {
        let mut neighbors = Vec::with_capacity(4);

        let Coordination { x, y } = self.coordination;

        match x {
            0 => neighbors.push(Coordination { x: 1, y }),
            x if x == maze[0].len() - 1 => neighbors.push(Coordination { x: x - 1, y }),
            x => {
                neighbors.push(Coordination { x: x + 1, y });
                neighbors.push(Coordination { x: x - 1, y });
            }
        }

        match y {
            0 => neighbors.push(Coordination { x, y: 1 }),
            y if y == maze.len() - 1 => neighbors.push(Coordination { x, y: y - 1 }),
            y => {
                neighbors.push(Coordination { x, y: y + 1 });
                neighbors.push(Coordination { x, y: y - 1 });
            }
        }

        neighbors
    }
}

impl PartialOrd for Agent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.priority_points.partial_cmp(&self.priority_points)
    }
}

impl Ord for Agent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority_points.cmp(&self.priority_points)
    }
}

pub fn input_maze(width: usize, height: usize) -> Maze {
    let mut stdin = std::io::stdin().lines();

    let mut maze = Vec::with_capacity(height);

    for _ in 0..height {
        let maze_buffer: Vec<Tile> = stdin
            .next()
            .unwrap()
            .unwrap()
            .into_bytes()
            .into_iter()
            .map(|x| match x {
                b'1' => Tile::Wall,
                b'0' => Tile::Path,
                _ => panic!(),
            })
            .collect();

        assert_eq!(maze_buffer.len(), width);

        maze.push(maze_buffer);
    }

    maze
}

#[inline]
fn input_pair_of_usize() -> (usize, usize) {
    let mut stdin = std::io::stdin().lines();
    let maze_size_buffer: Vec<usize> = stdin
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(maze_size_buffer.len(), 2);

    (maze_size_buffer[0], maze_size_buffer[1])
}

pub fn input_maze_size() -> (usize, usize) {
    input_pair_of_usize()
}

#[inline]
fn input_coordination() -> Coordination {
    let (x, y) = input_pair_of_usize();
    Coordination { x, y }
}

pub fn input_starting_point() -> Coordination {
    input_coordination()
}

pub fn input_ending_point() -> Coordination {
    input_coordination()
}
