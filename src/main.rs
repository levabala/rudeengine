use num::{Float, Integer};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::cmp::PartialEq;
trait IntFloat: Integer + Float {}

struct Point<T: IntFloat> {
    x: T,
    y: T,
}

impl<T: IntFloat> PartialEq for Point<T> {
    fn eq(&self, p2: &Point<T>) -> bool {
        return self.x == p2.x && self.y == p2.y;
    }
}

impl<T: IntFloat> Distribution<Point<T>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point<T> {
        Point {
            x: rng.gen(),
            y: rng.gen(),
        }
    }
}

fn main() {
    // let p1 = Point::generate();

    // let p2 = Point::generate();

    // let are_they_equal = p1.equal(p2);

    // println!("Hello, world! {}", are_they_equal);
}
