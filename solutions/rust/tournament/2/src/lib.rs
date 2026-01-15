use crate::Outcome::{Draw, Loss, Win};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::iter::once;

pub fn tally(match_results: &str) -> String {
    let teams_map = match_results.lines().flat_map(MatchResult::try_from).fold(
        BTreeMap::new(),
        |mut acc, result| {
            let t1 = acc
                .entry(result.team1.clone())
                .or_insert_with(|| TeamScore::new(result.team1.clone()));
            t1.add_result(&result);

            let t2 = acc
                .entry(result.team2.clone())
                .or_insert_with(|| TeamScore::new(result.team2.clone()));
            t2.add_result(&result);

            acc
        },
    );

    let mut teams_results: Vec<_> = teams_map.values().collect();
    teams_results.sort_by(|a, b| team_ordering(a, b));

    once("Team                           | MP |  W |  D |  L |  P".to_string())
        .chain(teams_results.iter().map(|score| score.to_string()))
        .collect::<Vec<_>>()
        .join("\n")
}

fn team_ordering(team1: &TeamScore, team2: &TeamScore) -> Ordering {
    match team2.points().cmp(&team1.points()) {
        Ordering::Equal => team1.name.cmp(&team2.name),
        o => o,
    }
}

#[derive(Debug)]
struct TeamScore {
    name: String,
    win: u32,
    draw: u32,
    loss: u32,
}
impl Display for TeamScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        const NAME_W: usize = 30;
        const NUM_W: usize = 2;

        write!(
            f,
            "{:<NAME_W$} | {:>NUM_W$} | {:>NUM_W$} | {:>NUM_W$} | {:>NUM_W$} | {:>NUM_W$}",
            self.name,
            self.win + self.draw + self.loss,
            self.win,
            self.draw,
            self.loss,
            self.points(),
        )
    }
}
impl TeamScore {
    fn new(name: String) -> Self {
        TeamScore {
            name,
            win: 0,
            draw: 0,
            loss: 0,
        }
    }

    fn points(&self) -> u32 {
        (self.win * 3) + self.draw
    }

    fn add_result(&mut self, result: &MatchResult) {
        match result.outcome_for(self.name.clone()) {
            Some(Win) => self.win += 1,
            Some(Draw) => self.draw += 1,
            Some(Loss) => self.loss += 1,
            None => (),
        }
    }
}

#[derive(Debug, Clone)]
enum Outcome {
    Win,
    Loss,
    Draw,
}
impl TryFrom<&str> for Outcome {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "win" => Ok(Win),
            "draw" => Ok(Draw),
            "loss" => Ok(Loss),
            &_ => Err(format!("Invalid outcome '{value}'")),
        }
    }
}

#[derive(Debug)]
struct MatchResult {
    team1: String,
    team2: String,
    outcome: Outcome,
}
impl Outcome {
    fn reverse(&self) -> Outcome {
        match self {
            Win => Loss,
            Loss => Win,
            Draw => Draw,
        }
    }
}
impl MatchResult {
    fn outcome_for(&self, team: String) -> Option<Outcome> {
        match (self.team1 == team, self.team2 == team) {
            (false, false) => None,
            (true, _) => Some(self.outcome.clone()),
            (_, true) => Some(self.outcome.reverse()),
        }
    }
}

impl TryFrom<&str> for MatchResult {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let x = value.split(';').collect::<Vec<_>>();
        if x.len() >= 3 {
            let team1 = x[0].to_string();
            let team2 = x[1].to_string();
            let result = Outcome::try_from(x[2])?;
            Ok(MatchResult {
                team1,
                team2,
                outcome: result,
            })
        } else {
            Err(format!("invalid result {value}"))
        }
    }
}
