pub fn part_one(input: &str) -> Option<u32> {
    let mut puzzleMatrix: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        let dataline: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        puzzleMatrix.push(dataline);
    }

    // Algo here

    let mut visibleTrees: u32 = 0;
    let columnsize = puzzleMatrix[0].len();

    for row in 1..puzzleMatrix.len() - 1 {
        for column in 1..columnsize - 1 {
            let element = puzzleMatrix[row][column];

            // Top element

            // To the left
            let left_elements: u32 = (puzzleMatrix[row])[0..column]
                .into_iter()
                .filter(|&x| x >= &element)
                .count() as u32;

            // To the right
            let right_elements: u32 = (puzzleMatrix[row])[column + 1..columnsize]
                .into_iter()
                .filter(|&x| x >= &element)
                .count() as u32;

            let top_elements: u32 = puzzleMatrix[0..row]
                .into_iter()
                .filter(|x| x[column] >= element)
                .count() as u32;

            let bottom_elements: u32 = puzzleMatrix[row + 1..puzzleMatrix.len()]
                .into_iter()
                .filter(|x| x[column] >= element)
                .count() as u32;

            if left_elements == 0
                || right_elements == 0
                || bottom_elements == 0
                || top_elements == 0
            {
                visibleTrees += 1;
            }
        }
    }

    return Some(visibleTrees + (2 * (columnsize as u32 + puzzleMatrix.len() as u32)) - 4);
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
