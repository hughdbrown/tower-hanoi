pub tower;

use tower::{
    Tower,
};

fn main() {
    let mut tower: Tower = Tower::new(4i8);
    tower.run();
    tower.visualize();
}
