use num::{Float, Integer};
use rand::distributions::{Distribution, Standard};
use rand::{random, Rng};
use std::cmp::PartialEq;

trait IntFloat: Integer + Float {}

struct Point<T> {
    x: T,
    y: T,
}

// TODO: unite comparing and distribution for Point i32 and f32
impl PartialEq for Point<i32> {
    fn eq(&self, p2: &Point<i32>) -> bool {
        return self.x == p2.x && self.y == p2.y;
    }
}

impl PartialEq for Point<f32> {
    fn eq(&self, p2: &Point<f32>) -> bool {
        return self.x == p2.x && self.y == p2.y;
    }
}

impl Distribution<Point<i32>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point<i32> {
        Point {
            x: rng.gen(),
            y: rng.gen(),
        }
    }
}

impl Distribution<Point<f32>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point<f32> {
        Point {
            x: rng.gen(),
            y: rng.gen(),
        }
    }
}

impl Point<f32> {
    fn floor(&self) -> Point<i32> {
        Point {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

fn main() {
    let p1 = random::<Point<i32>>();
    let p2 = random::<Point<i32>>();

    let p3 = random::<Point<f32>>();
    let p4 = random::<Point<f32>>();

    let p5 = p4.floor();

    let c1 = p1 == p2;
    let c2 = p3 == p4;
}
