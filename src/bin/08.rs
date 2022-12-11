pub fn part_one(input: &str) -> Option<u32> {
    let mut puzzle_matrix: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        let dataline: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        puzzle_matrix.push(dataline);
    }

    // Algo here

    let mut visible_trees: u32 = 0;
    let columnsize = puzzle_matrix[0].len();

    for row in 1..puzzle_matrix.len() - 1 {
        for column in 1..columnsize - 1 {
            let element = puzzle_matrix[row][column];

            // Top element

            // To the left
            let left_elements: u32 = (puzzle_matrix[row])[0..column]
                .into_iter()
                .filter(|&x| x >= &element)
                .count() as u32;

            // To the right
            let right_elements: u32 = (puzzle_matrix[row])[column + 1..columnsize]
                .into_iter()
                .filter(|&x| x >= &element)
                .count() as u32;

            let top_elements: u32 = puzzle_matrix[0..row]
                .into_iter()
                .filter(|x| x[column] >= element)
                .count() as u32;

            let bottom_elements: u32 = puzzle_matrix[row + 1..puzzle_matrix.len()]
                .into_iter()
                .filter(|x| x[column] >= element)
                .count() as u32;

            if left_elements == 0
                || right_elements == 0
                || bottom_elements == 0
                || top_elements == 0
            {
                visible_trees += 1;
            }
        }
    }

    return Some(visible_trees + (2 * (columnsize as u32 + puzzle_matrix.len() as u32)) - 4);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut puzzle_matrix: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        let dataline: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        puzzle_matrix.push(dataline);
    }

    // Algo here

    let mut scenic_scores: Vec<u32> = Vec::new();
    let columnsize = puzzle_matrix[0].len();

    for row in 1..puzzle_matrix.len() - 1 {
        for column in 1..columnsize - 1 {
            let element = puzzle_matrix[row][column];

            // Top element

            // To the left
            let left_elements: usize = (puzzle_matrix[row])[0..column]
                .into_iter()
                .rev()
                .position(|x| x >= &element)
                .unwrap_or_else(|| column - 1)
                + 1;

            // To the right
            let right_elements: usize = (puzzle_matrix[row])[column + 1..]
                .into_iter()
                .position(|x| x >= &element)
                .unwrap_or_else(|| columnsize - column - 2)
                + 1;

            let top_elements: usize = puzzle_matrix[0..row]
                .into_iter()
                .rev()
                .position(|x| x[column] >= element)
                .unwrap_or_else(|| row - 1)
                + 1;

            let bottom_elements: usize = puzzle_matrix[row + 1..]
                .into_iter()
                .position(|x| x[column] >= element)
                .unwrap_or_else(|| puzzle_matrix.len() - row - 2)
                + 1;

            /*println!(
                "element: {}, left: {}, right: {}, top: {}, bottom: {}",
                element, left_elements, right_elements, top_elements, bottom_elements
            );*/

            scenic_scores
                .push((left_elements * right_elements * top_elements * bottom_elements) as u32);
        }
    }

    return scenic_scores.iter().copied().max();
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
