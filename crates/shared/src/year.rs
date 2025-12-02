use std::collections::HashMap;

use crate::Problem;

pub trait Year {
    fn problems(&self) -> HashMap<u8, Box<dyn Problem>>;
    fn year(&self) -> u32;
}
