mod framework;
use framework::game::{Game, MutableGame, GameImpl};

fn main() {
    println!("Hello, world!");
}

enum Resource {
    Wool,
    Grain,
    Ore,
    Lumber,
    Brick
}
