pub fn get_score(game: &str) -> u32 {
    let mut game_components = game.split(" ");
    let opponent_choice = game_components.next().unwrap();
    let own_choice = game_components.next().unwrap();

    let score = match opponent_choice {
        "A" => {
            if own_choice == "Y" {
                6
            } else if own_choice == "X" {
                3
            } else {
                0
            }
        }
        "B" => {
            if own_choice == "Z" {
                6
            } else if own_choice == "Y" {
                3
            } else {
                0
            }
        }
        "C" => {
            if own_choice == "X" {
                6
            } else if own_choice == "Z" {
                3
            } else {
                0
            }
        }
        _ => 0,
    };

    let own_increment = match own_choice {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    };

    return score + own_increment;
}

fn get_score2(game: &str) -> u32 {
    let mut game_components = game.split(" ");
    let opponent_choice = game_components.next().unwrap();
    let game_goal = game_components.next().unwrap();

    let score = match game_goal {
        "X" => {
            // Verlieren: 0+
            if opponent_choice == "A" {
                0 + 3
            } else if opponent_choice == "B" {
                0 + 1
            } else {
                0 + 2
            }
        }
        "Y" => {
            // Untenschieden: 3+
            if opponent_choice == "A" {
                3 + 1
            } else if opponent_choice == "B" {
                3 + 2
            } else {
                3 + 3
            }
        }
        "Z" => {
            // Gewinnen: 6+
            if opponent_choice == "A" {
                6 + 2
            } else if opponent_choice == "B" {
                6 + 3
            } else {
                6 + 1
            }
        }
        _ => 0,
    };

    return score;
}

pub fn part_one(input: &str) -> Option<u32> {
    return Some(input.lines().fold(0, |prev, x| prev + get_score(x)));
}

pub fn part_two(input: &str) -> Option<u32> {
    return Some(input.lines().fold(0, |prev, x| prev + get_score2(x)));
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
