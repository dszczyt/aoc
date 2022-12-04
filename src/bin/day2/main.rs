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
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissor,
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
    // pub my_choice: Choice,
    pub opponent_choice: Choice,
    pub expected_result: RoundResult,
}

impl From<&str> for Round {
    fn from(input: &str) -> Self {
        let split_input: Vec<&str> = input.split(' ').collect();
        Self {
            expected_result: split_input.get(1).unwrap().clone().into(),
            opponent_choice: split_input.get(0).unwrap().clone().into(),
        }
    }
}

impl Round {
    pub fn score(self) -> Score {
        let my_choice: Choice = self.into();
        // let result: RoundResult = self.into();
        let result_score: Score = my_choice.into();
        let expected_result: Score = self.expected_result.into();

        result_score + expected_result
    }
}

#[derive(Clone, Copy, Debug)]
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

impl From<&str> for RoundResult {
    fn from(input: &str) -> Self {
        match input {
            "X" => Self::Defeat,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!(),
        }
    }
}

impl From<Round> for Choice {
    fn from(round: Round) -> Self {
        match (round.opponent_choice, round.expected_result) {
            (Choice::Rock, RoundResult::Win) => Self::Paper,
            (Choice::Rock, RoundResult::Draw) => Self::Rock,
            (Choice::Rock, RoundResult::Defeat) => Self::Scissor,
            (Choice::Paper, RoundResult::Win) => Self::Scissor,
            (Choice::Paper, RoundResult::Draw) => Self::Paper,
            (Choice::Paper, RoundResult::Defeat) => Self::Rock,
            (Choice::Scissor, RoundResult::Win) => Self::Rock,
            (Choice::Scissor, RoundResult::Draw) => Self::Scissor,
            (Choice::Scissor, RoundResult::Defeat) => Self::Paper,
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
    use crate::StrategyGuide;

    #[test]
    fn example() {
        let example = include_str!("example");
        let strategy_guide: StrategyGuide = example.into();
        assert_eq!(strategy_guide.score(), 12)
    }
}

fn main() {
    let input = include_str!("input");
    let strategy_guide: StrategyGuide = input.into();
    let score = strategy_guide.score();

    dbg!(&score);
}
