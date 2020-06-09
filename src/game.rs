use rand::distributions::{Distribution, Uniform};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Entity {
    Empty,
    Dog,
    Rabbit,
}
use Entity::*;

impl std::ops::Neg for Entity {
    type Output = Self;
    fn neg(self) -> Self {
        match self {
            Empty => Empty,
            Dog => Rabbit,
            Rabbit => Dog,
        }
    }
}

/// ```
/// . D-.-. .
///  /|\|/|\
/// D-.-.-.-R
///  \|/|\|/
/// . D . . .
/// ```

#[derive(Debug, Clone)]
pub struct Game {
    data: Vec<Vec<Entity>>,
}
impl Game {
    pub fn read() -> Self {
        let data = (0..3)
            .map(|_| {
                let mut line = String::new();
                let _ = io::stdin().read_line(&mut line);
                line.trim()
                    .chars()
                    .map(|c| match c {
                        'D' => Dog,
                        'R' => Rabbit,
                        _ => Empty,
                    })
                    .collect()
            })
            .collect();
        Self { data }
    }

    pub fn write(&self) {
        for i in 0..3 {
            for j in 0..5 {
                print!(
                    "{}",
                    match self.data[i][j] {
                        Empty => '.',
                        Dog => 'D',
                        Rabbit => 'R',
                    }
                );
            }
            println!();
        }
    }

    pub fn mirror(&self) -> Self {
        Self {
            data: self
                .data
                .iter()
                .map(|row| row.iter().rev().cloned().collect())
                .collect(),
        }
    }
}

fn neighbors(c: Entity, i: usize, j: usize) -> Vec<(usize, usize)> {
    match c {
        Empty => vec![],
        Dog => match (i, j) {
            (0, 1) => vec![(1, 1), (1, 2), (0, 2)],
            (0, 2) => vec![(1, 2), (0, 3)],
            (0, 3) => vec![(1, 3)],
            (1, 0) => vec![(0, 1), (1, 1), (2, 1)],
            (1, 1) => vec![(0, 1), (1, 2), (2, 1)],
            (1, 2) => vec![(0, 2), (0, 3), (1, 3), (2, 2), (2, 3)],
            (1, 3) => vec![(0, 3), (2, 3)],
            (2, 1) => vec![(1, 1), (1, 2), (2, 2)],
            (2, 2) => vec![(1, 2), (2, 3)],
            (2, 3) => vec![(1, 3)],
            _ => vec![],
        },
        Rabbit => match (i, j) {
            (0, 1) => vec![],
            (0, 2) => vec![(0, 1), (1, 2), (0, 3)],
            (0, 3) => vec![(0, 2), (1, 2), (1, 3), (1, 4)],
            (1, 0) => vec![],
            (1, 1) => vec![(0, 1), (1, 0), (1, 2), (2, 1)],
            (1, 2) => vec![
                (0, 1),
                (0, 2),
                (0, 3),
                (1, 1),
                (1, 3),
                (2, 1),
                (2, 2),
                (2, 3),
            ],
            (1, 3) => vec![(0, 3), (1, 2), (1, 4), (2, 3)],
            (1, 4) => vec![(0, 3), (1, 3), (2, 3)],
            (2, 1) => vec![],
            (2, 2) => vec![(2, 1), (1, 2), (2, 3)],
            (2, 3) => vec![(2, 2), (1, 2), (1, 3), (1, 4)],
            _ => vec![],
        },
    }
}

#[derive(Debug, Clone)]
enum GameState {
    InGame,
    DogWin,
    RabbitWin,
}
use GameState::*;

fn judge(g: &Game) -> GameState {
    if g.data[0][1] == Rabbit || g.data[1][0] == Rabbit || g.data[2][1] == Rabbit {
        return RabbitWin;
    }
    if g.data[1][0] == Empty && g.data[1][1] == Rabbit {
        return RabbitWin;
    }
    if (g.data[0][3], g.data[1][3], g.data[2][3], g.data[1][4]) == (Dog, Dog, Dog, Rabbit) {
        return DogWin;
    }
    InGame
}

fn choices(g: &Game, next: Entity) -> Vec<((usize, usize), (usize, usize))> {
    if next == Empty {
        return vec![];
    }
    let mut ways = vec![];
    for i in 0..3 {
        for j in 0..5 {
            if g.data[i][j] != next {
                continue;
            }
            for &(u, v) in neighbors(next, i, j).iter() {
                if g.data[u][v] != Empty {
                    continue;
                }
                ways.push(((i, j), (u, v)));
            }
        }
    }
    ways
}

/// Move: (i, j) to (u, v)
fn play(g: &Game, choice: ((usize, usize), (usize, usize))) -> Game {
    let ((i, j), (u, v)) = choice;
    let mut h = g.clone();
    h.data[u][v] = g.data[i][j];
    h.data[i][j] = Empty;
    h
}

fn play_trivial(g: &Game, next: Entity) -> Option<Game> {
    let ways = choices(g, next);
    for &choice in ways.iter() {
        let h = play(g, choice);
        // trivial case
        match (next, judge(&h)) {
            (Dog, DogWin) | (Rabbit, RabbitWin) => return Some(h),
            _ => {}
        }
    }
    None
}

fn play_random(g: &Game, next: Entity) -> Option<Game> {
    let ways = choices(g, next);
    // cant move
    if ways.is_empty() {
        return None;
    }
    // trivial
    if let Some(g) = play_trivial(g, next) {
        return Some(g);
    }
    // random choice
    let mut rng = rand::thread_rng();
    let idx = Uniform::from(0..ways.len()).sample(&mut rng);
    Some(play(g, ways[idx]))
}

/// Rollplay with random_play
fn rollplay_random(g: &Game, first: Entity) -> GameState {
    let mut next = first;
    let mut h = g.clone();
    for _ in 0..30 {
        if let Some(h_next) = play_random(&h, next) {
            h = h_next;
            next = -next;
        } else {
            return judge(&h);
        }
    }
    InGame
}

fn play_good(g: &Game, next: Entity, depth: usize) -> Option<Game> {
    if depth >= 3 {
        return play_random(g, next);
    }
    let ways = choices(g, next);
    // cant move
    if ways.is_empty() {
        return None;
    }
    // trivial
    if let Some(gp) = play_trivial(g, next) {
        return Some(gp);
    }
    // scan next games
    let mut goodgame = None;
    let mut best_prob = -1.0;
    for &choice in ways.iter() {
        let h = play(g, choice);
        let q = 1.0 - prob_to_win(&h, -next, 100);
        if q > best_prob {
            best_prob = q;
            goodgame = Some(h);
        }
    }
    goodgame
}

/// Rollplay with random_play
fn rollplay_good(g: &Game, first: Entity) -> GameState {
    // println!("### RollplayGood first={:?}", first);
    let mut next = first;
    let mut h = g.clone();
    for _ in 0..30 {
        // println!();
        // h.write();
        if let Some(h_next) = play_good(&h, next, 0) {
            h = h_next;
            next = -next;
        } else {
            // println!("=> {:?}", judge(&h));
            return judge(&h);
        }
    }
    InGame
}

/// Winning Probabilities
///   estimated with play_random
fn prob_to_win(g: &Game, first: Entity, num_try: usize) -> f32 {
    if first == Empty {
        return 0.5;
    }
    let mut win = 0;
    for _ in 0..num_try {
        match (first, rollplay_random(&g, first)) {
            (Dog, DogWin) | (Rabbit, RabbitWin) => {
                win += 1;
            }
            _ => {}
        }
    }
    win as f32 / num_try as f32
}

/// Winning Probabilities
///   estimated with play_good
fn prob_to_win_good(g: &Game, first: Entity, num_try: usize) -> f32 {
    let mut win = 0;
    for _ in 0..num_try {
        match (first, rollplay_good(&g, first)) {
            (Dog, DogWin) | (Rabbit, RabbitWin) => {
                win += 1;
            }
            _ => {}
        }
    }
    win as f32 / num_try as f32
}

pub fn solve(g: &Game, next: Entity, verbose: bool) -> Option<Game> {
    let mut goodgame = None;
    let mut prob = 0.0;
    for &choice in choices(g, next).iter() {
        let h = play(g, choice);
        let p = 1.0 - prob_to_win_good(&h, -next, 200);
        if verbose {
            println!("Choice {:?} => {:?}", choice.0, choice.1);
            h.write();
            println!("Prob: {:.3}", p);
            println!("---");
        }
        if p > prob {
            prob = p;
            goodgame = Some(h);
        }
    }
    goodgame
}
