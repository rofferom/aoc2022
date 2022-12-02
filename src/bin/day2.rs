const INPUT: &str = include_str!("day2_input.txt");

#[derive(Clone, Copy)]
pub enum Action {
    Rock,
    Paper,
    Scissor,
}

fn play(actions: Vec<(Action, Action)>) -> u32 {
    actions
        .into_iter()
        .map(|(opponent, mine)| {
            let action_value = match mine {
                Action::Rock => 1,
                Action::Paper => 2,
                Action::Scissor => 3,
            };

            let outcome = match (opponent, mine) {
                (Action::Rock, Action::Rock) => 3,
                (Action::Rock, Action::Paper) => 6,
                (Action::Rock, Action::Scissor) => 0,

                (Action::Paper, Action::Rock) => 0,
                (Action::Paper, Action::Paper) => 3,
                (Action::Paper, Action::Scissor) => 6,

                (Action::Scissor, Action::Rock) => 6,
                (Action::Scissor, Action::Paper) => 0,
                (Action::Scissor, Action::Scissor) => 3,
            };

            action_value + outcome
        })
        .sum()
}

mod part1 {
    use super::Action;

    pub fn parse_input(input: &str) -> Vec<(Action, Action)> {
        input
            .lines()
            .map(|l| {
                let actions: Vec<_> = l
                    .split(' ')
                    .map(|s| match s {
                        "A" | "X" => Action::Rock,
                        "B" | "Y" => Action::Paper,
                        "C" | "Z" => Action::Scissor,
                        _ => panic!("Invalid Action {s}"),
                    })
                    .collect();
                (actions[0], actions[1])
            })
            .collect()
    }
}

mod part2 {
    use super::Action;

    enum Outcome {
        Lose,
        Draw,
        Win,
    }

    pub fn parse_input(input: &str) -> Vec<(Action, Action)> {
        input
            .lines()
            .map(|l| {
                let l: Vec<_> = l.split(' ').collect();
                let (str_opponent, str_outcome) = (l[0], l[1]);

                let opponent = match str_opponent {
                    "A" => Action::Rock,
                    "B" => Action::Paper,
                    "C" => Action::Scissor,
                    _ => panic!("Invalid Action {str_opponent}"),
                };

                let outcome = match str_outcome {
                    "X" => Outcome::Lose,
                    "Y" => Outcome::Draw,
                    "Z" => Outcome::Win,
                    _ => panic!("Invalid Outcome {str_outcome}"),
                };

                let action = match (opponent, outcome) {
                    (Action::Paper, Outcome::Lose) => Action::Rock,
                    (Action::Paper, Outcome::Draw) => Action::Paper,
                    (Action::Paper, Outcome::Win) => Action::Scissor,

                    (Action::Rock, Outcome::Lose) => Action::Scissor,
                    (Action::Rock, Outcome::Draw) => Action::Rock,
                    (Action::Rock, Outcome::Win) => Action::Paper,

                    (Action::Scissor, Outcome::Lose) => Action::Paper,
                    (Action::Scissor, Outcome::Draw) => Action::Scissor,
                    (Action::Scissor, Outcome::Win) => Action::Rock,
                };

                (opponent, action)
            })
            .collect()
    }
}

fn solve_part1(input: &str) -> u32 {
    let actions = part1::parse_input(input);
    play(actions)
}

fn solve_part2(input: &str) -> u32 {
    let actions = part2::parse_input(input);
    play(actions)
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn day1() {
        assert_eq!(solve_part1(TEST_INPUT), 15);
        assert_eq!(solve_part1(INPUT), 12645);

        assert_eq!(solve_part2(TEST_INPUT), 12);
        assert_eq!(solve_part2(INPUT), 11756);
    }
}
