use rand::Rng;

// here I added `pub` notation because it's better practice to have all tests in
// `test/` directory to make code cleaner
pub const TIMESTAMPS_COUNT: usize = 50_000;

pub const PROBABILITY_SCORE_CHANGED: f64 = 0.0_001;

pub const PROBABILITY_HOME_SCORE: f64 = 0.45;

pub const OFFSET_MAX_STEP: i32 = 3;

pub const INITIAL_STAMP: Stamp = Stamp {
    offset: 0,
    score: Score { home: 0, away: 0 },
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Score {
    pub home: i32,
    pub away: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stamp {
    pub offset: i32,
    pub score: Score,
}

// it would be more memory efficient if we use links (&Stamp) instead of
// moving value to function
pub fn generate_stamp(previous_value: Stamp) -> Stamp {
    let score_changed: bool = rand::thread_rng().gen_bool(PROBABILITY_SCORE_CHANGED);
    let home_score_change: bool = rand::thread_rng().gen_bool(PROBABILITY_HOME_SCORE);
    let offset_change: i32 = rand::thread_rng().gen_range(1..=OFFSET_MAX_STEP);

    Stamp {
        offset: previous_value.offset + offset_change,
        score: Score {
            home: previous_value.score.home
                + if score_changed && home_score_change {
                    1
                } else {
                    0
                },
            away: previous_value.score.away
                + if score_changed && !home_score_change {
                    1
                } else {
                    0
                },
        },
    }
}

pub fn generate_game() -> Vec<Stamp> {
    let mut stamps = vec![INITIAL_STAMP];
    let mut current_stamp = INITIAL_STAMP;

    for _ in 0..TIMESTAMPS_COUNT {
        current_stamp = generate_stamp(current_stamp);
        stamps.push(current_stamp);
    }

    stamps
}

pub fn get_score(game_stamps: &[Stamp], offset: i32) -> (i32, i32) {
    let score = game_stamps
        .iter()
        .find(|stamp| {
            stamp.offset == offset
        })
        .map(|stamp| (stamp.score.home, stamp.score.away))
        .unwrap_or((-1, -1)); // in Rust it would be better to use Result<Stamp, String/Error> or Option<Stamp> as a return type
                                     // but here we want to know about Error having backward compatability

    score
}

