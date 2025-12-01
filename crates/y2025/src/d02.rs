use shared::Problem;

fn parse_input(contents: &str) -> () {
    for line in contents.trim().lines() {}
    todo!()
}

fn compute_1(contents: &str) -> u64 {
    let _x = parse_input(contents);
    todo!()
}

fn compute_2(contents: &str) -> u64 {
    let x = parse_input(contents);
    todo!()
}

pub(crate) struct Day {}

impl Problem for Day {
    fn source_code_file(&self) -> String {
        file!().to_string()
    }
    fn solve1(&self, contents: &str) -> String {
        format!("{}", compute_1(contents))
    }
    // fn expected1(&self) -> String {
    //     "abc".to_string()
    // }
    // fn solve2(&self, contents: &str) -> String {
    //     format!("{}", compute_2(contents))
    // }
    // fn expected2(&self) -> String {
    //     "xyz".to_string()
    // }
}
