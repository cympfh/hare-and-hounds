mod game;
use game::{solve, Entity, Game};

extern crate structopt;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(short, long)]
    verbose: bool,
    #[structopt(short, long)]
    mirror: bool,
    #[structopt(short, long)]
    next: String,
}

fn main() {
    let opt = Opt::from_args();
    let mut g = Game::read();
    if opt.mirror {
        g = g.mirror();
    }
    let next = match opt.next.as_str() {
        "D" | "Dog" | "dog" | "d" => Entity::Dog,
        _ => Entity::Rabbit,
    };
    if let Some(goodgame) = solve(&g, next, opt.verbose) {
        let g = if opt.mirror {
            goodgame.mirror()
        } else {
            goodgame
        };
        g.write();
    } else {
        println!("{}", -1);
    }
}
