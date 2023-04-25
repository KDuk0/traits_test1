use std::fmt::{Debug, Display, Formatter, Result};

trait Fruitiness {
    fn is_sweet(&self) -> bool {
        self.sweetness() >= 0.5
    }
    fn sweetness(&self) -> f32;
}

struct Pear {}
struct Lemon {}

impl Fruitiness for Pear {
    fn sweetness(&self) -> f32 {
        0.6
    }
}

impl Display for Pear {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("A pear")
    }
}

impl Debug for Pear {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("A debugged pear")
    }
}

impl Fruitiness for Lemon {
    fn sweetness(&self) -> f32 {
        0.2
    }
}

impl Display for Lemon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("A lemon")
    }
}

impl Debug for Lemon {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("A debugged lemon")
    }
}

fn print_sweetness<T>(fruit: T) where
T: Fruitiness + Display + Debug,
{
    println!("{} is sweet? {}", fruit, fruit.is_sweet());
}

fn main() {
    let pear = Pear {};
    let lemon = Lemon {};
    print_sweetness(pear);
    print_sweetness(lemon);
}