use tasks::first_task::*;

#[test]
fn cargo_test_check() {
    assert_eq!(1,1)
}

const MOCK_TIMESTAMP_COUNT: usize = 10;


fn new_home_score_stamp(prev: &Stamp) -> Stamp {
    Stamp {
        offset: prev.offset + 2,
        score: Score {
            home: prev.score.home + 1,
            away: prev.score.away,
        }
    }
}

fn new_away_score_stamp(prev: &Stamp) -> Stamp {
    Stamp {
        offset: prev.offset + 1,
        score: Score {
            home: prev.score.home,
            away: prev.score.away + 1,
        }
    }
}

fn mock_game_always_home() -> Vec<Stamp> {
    let mut stamps = vec![INITIAL_STAMP];
    let mut current_stamp = INITIAL_STAMP;

    for _ in 0..MOCK_TIMESTAMP_COUNT {
        current_stamp = new_home_score_stamp(&current_stamp);
        stamps.push(current_stamp);
    }

    stamps
}

fn mock_game_never_home() -> Vec<Stamp> {
    let mut stamps = vec![INITIAL_STAMP];
    let mut current_stamp = INITIAL_STAMP;

    for _ in 0..MOCK_TIMESTAMP_COUNT * 10 {
        current_stamp = new_away_score_stamp(&current_stamp);
        stamps.push(current_stamp);
    }

    stamps
}


// always new home score tests
#[test]
fn always_new_home_test() {
    let game = mock_game_always_home();
    assert_eq!(game.len(), MOCK_TIMESTAMP_COUNT + 1);
    assert_eq!(game[0],  Stamp { offset: 0, score: Score { home: 0, away: 0 }} );
    assert_eq!(game[10], Stamp { offset:20, score: Score { home:10, away: 0 }} );
}

#[test]
fn always_new_home_score_success_1_test() {
    let game = mock_game_always_home();
    assert_eq!((10, 0), get_score(&game, 20));
}

#[test]
fn always_new_home_score_success_2_test() {
    let game = mock_game_always_home();
    assert_eq!((7, 0), get_score(&game, 14));
}

#[test]
fn always_new_home_score_success_3_test() {
    let game = mock_game_always_home();
    assert_eq!((2, 0), get_score(&game, 4));
}

#[test]
fn always_new_home_score_success_4_test() {
    let game = mock_game_always_home();
    for i in 0..MOCK_TIMESTAMP_COUNT {
        assert_eq!((i as i32, 0), get_score(&game, i as i32 * 2))
    }
}

#[test]
fn always_new_home_score_fail_1_test() {
    let game = mock_game_always_home();
    assert_eq!((-1, -1), get_score(&game, 21));
}

#[test]
fn always_new_home_score_fail_2_test() {
    let game = mock_game_always_home();
    assert_eq!((-1, -1), get_score(&game, 19));
}

#[test]
fn always_new_home_score_fail_3_test() {
    let game = mock_game_always_home();
    assert_eq!((-1, -1), get_score(&game, -1));
}

#[test]
fn always_new_home_score_fail_4_test() {
    let game = mock_game_always_home();
    assert_eq!((-1, -1), get_score(&game, -1_000_000_000));
}

#[test]
fn always_new_home_score_fail_5_test() {
    let game = mock_game_always_home();
    assert_eq!((-1, -1), get_score(&game, 1_000_000_000));
}


// always new away score tests
#[test]
fn always_new_away_test() {
    let game = mock_game_never_home();
    assert_eq!(game.len(), MOCK_TIMESTAMP_COUNT * 10 + 1);
    assert_eq!(game[0],  Stamp { offset: 0, score: Score { home: 0, away: 0 }} );
    assert_eq!(game[10], Stamp { offset:10, score: Score { home: 0, away:10 }} );
}


#[test]
fn always_new_away_score_success_1_test() {
    let game = mock_game_never_home();
    assert_eq!((0, 10), get_score(&game, 10));
}

#[test]
fn always_new_away_score_success_2_test() {
    let game = mock_game_never_home();
    assert_eq!((0, 7), get_score(&game, 7));
}

#[test]
fn always_new_away_score_success_3_test() {
    let game = mock_game_never_home();
    assert_eq!((0, 2), get_score(&game, 2));
}

#[test]
fn always_new_away_score_success_4_test() {
    let game = mock_game_never_home();
    for i in 0..MOCK_TIMESTAMP_COUNT {
        assert_eq!((0, i as i32), get_score(&game, i as i32))
    }
}

#[test]
fn always_new_away_score_fail_1_test() {
    let game = mock_game_never_home();
    assert_eq!((-1, -1), get_score(&game, -1));
}

#[test]
fn always_new_away_score_fail_2_test() {
    let game = mock_game_never_home();
    assert_eq!((-1, -1), get_score(&game, -100000002));
}

#[test]
fn always_new_away_score_fail_3_test() {
    let game = mock_game_never_home();
    assert_eq!((-1, -1), get_score(&game, 105));
}

// 50% home and 50% away change

fn mock_game(count: usize) -> Vec<Stamp> {
    let mut game = vec![INITIAL_STAMP];
    let mut cur_stamp = INITIAL_STAMP;

    for i in 0..count {
        cur_stamp = if i % 2 == 0 {
            new_home_score_stamp(&cur_stamp)
        } else {
            new_away_score_stamp(&cur_stamp)
        };
        game.push(cur_stamp);
    }

    game
}


#[test]
fn mock_game_test() {
    let game = mock_game(MOCK_TIMESTAMP_COUNT * 2);
    assert_eq!(game.len(), MOCK_TIMESTAMP_COUNT * 2 + 1);
    println!("{:#?}", game);
    assert_eq!(game[0], INITIAL_STAMP);
    assert_eq!(game[4], Stamp { offset: 6, score: Score { home: 2, away: 2 }} );
    assert_eq!(game[8], Stamp { offset: 12, score: Score { home: 4, away: 4 }} );
    assert_eq!(game[17], Stamp { offset: 26, score: Score { home: 9, away: 8 }} );
}


#[test]
fn mock_game_success_test() {
    let game = mock_game(MOCK_TIMESTAMP_COUNT * 2);
    let mut result = (0, 0);
    let mut step = 0;

    for i in 0..MOCK_TIMESTAMP_COUNT * 2 {
        if i != 0 && i % 2 == 1 {
            result = (result.0 + 1, result.1);
        }
        if i != 0 && i % 2 == 0 {
            result = (result.0, result.1 + 1);
            step += 1;
        }

        assert_eq!(result, get_score(&game, i as i32 * 2 - step));
    }
}

#[test]
fn mock_game_failure_1_test() {
    let game = mock_game(MOCK_TIMESTAMP_COUNT * 2);
    assert_eq!((-1, -1), get_score(&game, 1_000_000));
}

#[test]
fn mock_game_failure_2_test() {
    let game = mock_game(MOCK_TIMESTAMP_COUNT * 2);
    assert_eq!((-1, -1), get_score(&game, -1_000_000));
}

#[test]
fn mock_game_failure_3_test() {
    let game = mock_game(MOCK_TIMESTAMP_COUNT * 2);
    assert_eq!((-1, -1), get_score(&game, 22));
}

#[test]
fn mock_game_failure_4_test() {
    let game = mock_game(MOCK_TIMESTAMP_COUNT * 2);
    assert_eq!((-1, -1), get_score(&game, 10));
}

#[test]
fn mock_game_failure_5_test() {
    let game = mock_game(MOCK_TIMESTAMP_COUNT * 2);
    assert_eq!((-1, -1), get_score(&game, 31));
}