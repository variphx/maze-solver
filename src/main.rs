use maze_solver::*;
use std::io::Write;

fn main() {
    let solver = {
        let maze = {
            let (width, height) = input_maze_size();
            input_maze(width, height)
        };
        let starting_point = input_starting_point();
        let ending_point = input_ending_point();

        MazeSolver::new(maze, starting_point, ending_point)
    };

    let solution = solver.solution().unwrap();

    let mut stdout = std::io::stdout().lock();

    for step in solution.windows(2) {
        let (before, after) = (step[0], step[1]);
        match (
            after.x() as isize - before.x() as isize,
            after.y() as isize - before.y() as isize,
        ) {
            (-1, _) => writeln!(&mut stdout, "left").unwrap(),
            (_, -1) => writeln!(&mut stdout, "up").unwrap(),
            (1, _) => writeln!(&mut stdout, "right").unwrap(),
            (_, 1) => writeln!(&mut stdout, "down").unwrap(),
            _ => panic!(),
        }
    }
}
