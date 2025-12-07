use std::collections::HashMap;

use shared::{Problem, Year as YearTrait};

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
// mod d07;
// mod d08;
// mod d09;
// mod d10;
// mod d11;
// mod d12;
// mod d13;
// mod d14;
// mod d15;
// mod d16;
// mod d17;
// mod d18;
// mod d19;
// mod d20;
// mod d21;
// mod d22;
// mod d23;
// mod d24;
// mod d25;

pub struct Year {}

impl YearTrait for Year {
    fn problems(&self) -> HashMap<u8, Box<dyn Problem>> {
        let mut problems: HashMap<u8, Box<dyn Problem>> = HashMap::new();

        problems.insert(1, Box::new(d01::Day {}));
        problems.insert(2, Box::new(d02::Day {}));
        problems.insert(3, Box::new(d03::Day {}));
        problems.insert(4, Box::new(d04::Day {}));
        problems.insert(5, Box::new(d05::Day {}));
        problems.insert(6, Box::new(d06::Day {}));
        // problems.insert(7, Box::new(d07::Day {}));
        // problems.insert(8, Box::new(d08::Day {}));
        // problems.insert(9, Box::new(d09::Day {}));
        // problems.insert(10, Box::new(d10::Day {}));
        // problems.insert(11, Box::new(d11::Day {}));
        // problems.insert(12, Box::new(d12::Day {}));
        // problems.insert(13, Box::new(d13::Day {}));
        // problems.insert(14, Box::new(d14::Day {}));
        // problems.insert(15, Box::new(d15::Day {}));
        // problems.insert(16, Box::new(d16::Day {}));
        // problems.insert(17, Box::new(d17::Day {}));
        // problems.insert(18, Box::new(d18::Day {}));
        // problems.insert(19, Box::new(d19::Day {}));
        // problems.insert(20, Box::new(d20::Day {}));
        // problems.insert(21, Box::new(d21::Day {}));
        // problems.insert(22, Box::new(d22::Day {}));
        // problems.insert(23, Box::new(d23::Day {}));
        // problems.insert(24, Box::new(d24::Day {}));
        // problems.insert(25, Box::new(d25::Day {}));

        problems
    }

    fn year(&self) -> u32 {
        2025
    }
}
