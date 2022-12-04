type Score = usize;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

impl From<&str> for Choice {
    fn from(input: &str) -> Self {
        match input {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissor,
            _ => unreachable!(),
        }
    }
}

impl From<Choice> for Score {
    fn from(choice: Choice) -> Self {
        match choice {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissor => 3,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Round {
    pub my_choice: Choice,
    pub opponent_choice: Choice,
}

impl From<&str> for Round {
    fn from(input: &str) -> Self {
        let choices: Vec<Choice> = input.split(' ').map(|input| input.into()).collect();
        Self {
            my_choice: choices.get(1).unwrap().clone(),
            opponent_choice: choices.get(0).unwrap().clone(),
        }
    }
}

impl Round {
    pub fn score(self) -> Score {
        let result: RoundResult = self.into();
        let result_score: Score = result.into();
        let my_choice_score: Score = self.my_choice.into();

        result_score + my_choice_score
    }
}

#[derive(Debug)]
enum RoundResult {
    Win,
    Draw,
    Defeat,
}

impl From<RoundResult> for Score {
    fn from(result: RoundResult) -> Self {
        match result {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Defeat => 0,
        }
    }
}

impl From<Round> for RoundResult {
    fn from(round: Round) -> Self {
        if round.opponent_choice == round.my_choice {
            return Self::Draw;
        }

        match (round.opponent_choice, round.my_choice) {
            (Choice::Rock, Choice::Paper) => Self::Win,
            (Choice::Rock, Choice::Scissor) => Self::Defeat,
            (Choice::Paper, Choice::Rock) => Self::Defeat,
            (Choice::Paper, Choice::Scissor) => Self::Win,
            (Choice::Scissor, Choice::Rock) => Self::Win,
            (Choice::Scissor, Choice::Paper) => Self::Defeat,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct StrategyGuide {
    pub rounds: Vec<Round>,
}

impl StrategyGuide {
    pub fn score(self) -> Score {
        self.rounds.iter().map(|round| round.score()).sum()
    }
}

impl From<&str> for StrategyGuide {
    fn from(input: &str) -> Self {
        let rounds: Vec<Round> = input.lines().map(|line| line.into()).collect();
        Self { rounds }
    }
}

#[cfg(test)]
mod test {
    use crate::{Score, StrategyGuide};

    #[test]
    fn example() {
        let example = include_str!("example");
        let strategy_guide: StrategyGuide = example.into();
        assert_eq!(strategy_guide.score(), 15 as Score)
    }
}

fn main() {
    let input = include_str!("input");
    let strategy_guide: StrategyGuide = input.into();
    let score = strategy_guide.score();

    dbg!(&score);
}
