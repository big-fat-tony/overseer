use std::fmt::{Debug, Formatter};

/// Base decision result used by pick & ban strategies
pub enum Decision {
    Choice(ChampionChoice),
    None,
}

/// Represents a specific champion choice, like Python's ChampionChoice.
#[derive(Clone, Copy)]
pub struct ChampionChoice {
    champion_id: i32,
}

impl ChampionChoice {
    pub fn new(champion_id: i32) -> Self {
        Self { champion_id }
    }

    pub fn champion_id(&self) -> i32 {
        self.champion_id
    }
}

/// Helper to construct a ChampionChoice wrapped as a Decision
pub fn choose(champion_id: i32) -> Decision {
    Decision::Choice(ChampionChoice::new(champion_id))
}

/// Helper to return "no decision"
pub fn no_decision() -> Decision {
    Decision::None
}

impl Decision {
    pub fn has_choice(&self) -> bool {
        matches!(self, Decision::Choice(_))
    }

    pub fn champion_id(&self) -> i32 {
        match self {
            Decision::Choice(c) => c.champion_id(),
            Decision::None => 0,
        }
    }
}

impl Debug for Decision {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Decision::Choice(c) => f
                .debug_tuple("ChampionChoice")
                .field(&c.champion_id())
                .finish(),
            Decision::None => write!(f, "NoDecision"),
        }
    }
}
