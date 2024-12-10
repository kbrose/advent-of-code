use std::fs;

fn parse_input(contents: &str) -> () {
    for _line in contents.trim().split('\n') {}
    todo!()
}

fn compute_1(contents: &str) -> u64 {
    let _x = parse_input(contents);
    todo!()
}

// fn compute_2(contents: &str) -> u64 {
//     let x = parse_input(contents);
//     todo!()
// }

fn main() {
    let contents =
        fs::read_to_string("inputs/d00.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    // assert_eq!(670984704, result);
    println!("part 1: {result}");

    // let result = compute_2(&contents);
    // assert_eq!(262775362119547, result);
    // println!("part 2: {result}");
}
