use shared::Problem;

enum State {
    Uninit(bool),
    M,
    U,
    L,
    LParen,
    N1_1(u64),
    N1_2(u64),
    N1_3(u64),
    Comma(u64),
    N2_1(u64, u64),
    N2_2(u64, u64),
    N2_3(u64, u64),
    D(bool),
    O(bool),
    N(bool),
    Apostrophe(bool),
    T(bool),
    LParenConditional(bool, bool),
}

fn handle_uninit(c: char, enabled: bool, handle_conditionals: bool) -> (State, Option<u64>) {
    if c == 'm' && enabled {
        (State::M, None)
    } else if handle_conditionals && c == 'd' {
        (State::D(enabled), None)
    } else {
        (State::Uninit(enabled), None)
    }
}

fn process(curr_state: State, c: char, handle_conditionals: bool) -> (State, Option<u64>) {
    match curr_state {
        State::Uninit(enabled) => handle_uninit(c, enabled, handle_conditionals),
        State::M => {
            if c == 'u' {
                (State::U, None)
            } else {
                handle_uninit(c, true, handle_conditionals)
            }
        }
        State::U => {
            if c == 'l' {
                (State::L, None)
            } else {
                handle_uninit(c, true, handle_conditionals)
            }
        }
        State::L => {
            if c == '(' {
                (State::LParen, None)
            } else {
                handle_uninit(c, true, handle_conditionals)
            }
        }
        State::LParen => {
            if c.is_ascii_digit() {
                (State::N1_1(c.to_string().parse().unwrap()), None)
            } else {
                handle_uninit(c, true, handle_conditionals)
            }
        }
        State::N1_1(n) => {
            if c.is_ascii_digit() {
                (
                    State::N1_2(n * 10 + c.to_string().parse::<u64>().unwrap()),
                    None,
                )
            } else if c == ',' {
                (State::Comma(n), None)
            } else {
                handle_uninit(c, true, handle_conditionals)
            }
        }
        State::N1_2(n) => {
            if c.is_ascii_digit() {
                (
                    State::N1_3(n * 10 + c.to_string().parse::<u64>().unwrap()),
                    None,
                )
            } else if c == ',' {
                (State::Comma(n), None)
            } else {
                handle_uninit(c, true, handle_conditionals)
            }
        }
        State::N1_3(n) => {
            if c == ',' {
                (State::Comma(n), None)
            } else {
                handle_uninit(c, true, handle_conditionals)
            }
        }
        State::Comma(n) => {
            if c.is_ascii_digit() {
                (State::N2_1(n, c.to_string().parse::<u64>().unwrap()), None)
            } else {
                handle_uninit(c, true, handle_conditionals)
            }
        }
        State::N2_1(n1, n2) => {
            if c.is_ascii_digit() {
                (
                    State::N2_2(n1, n2 * 10 + c.to_string().parse::<u64>().unwrap()),
                    None,
                )
            } else if c == ')' {
                (State::Uninit(true), Some(n1 * n2))
            } else {
                handle_uninit(c, true, handle_conditionals)
            }
        }
        State::N2_2(n1, n2) => {
            if c.is_ascii_digit() {
                (
                    State::N2_3(n1, n2 * 10 + c.to_string().parse::<u64>().unwrap()),
                    None,
                )
            } else if c == ')' {
                (State::Uninit(true), Some(n1 * n2))
            } else {
                handle_uninit(c, true, handle_conditionals)
            }
        }
        State::N2_3(n1, n2) => {
            if c == ')' {
                (State::Uninit(true), Some(n1 * n2))
            } else {
                handle_uninit(c, true, handle_conditionals)
            }
        }
        State::D(enabled) => {
            if c == 'o' {
                (State::O(enabled), None)
            } else {
                handle_uninit(c, enabled, handle_conditionals)
            }
        }
        State::O(enabled) => {
            if c == '(' {
                (State::LParenConditional(enabled, true), None)
            } else if c == 'n' {
                (State::N(enabled), None)
            } else {
                handle_uninit(c, enabled, handle_conditionals)
            }
        }
        State::N(enabled) => {
            if c == '\'' {
                (State::Apostrophe(enabled), None)
            } else {
                handle_uninit(c, enabled, handle_conditionals)
            }
        }
        State::Apostrophe(enabled) => {
            if c == 't' {
                (State::T(enabled), None)
            } else {
                handle_uninit(c, enabled, handle_conditionals)
            }
        }
        State::T(enabled) => {
            if c == '(' {
                (State::LParenConditional(enabled, false), None)
            } else {
                handle_uninit(c, enabled, handle_conditionals)
            }
        }
        State::LParenConditional(enabled, should_enable) => {
            if c == ')' {
                (State::Uninit(should_enable), None)
            } else {
                handle_uninit(c, enabled, handle_conditionals)
            }
        }
    }
}

fn compute(contents: &str, handle_conditionals: bool) -> u64 {
    let mut state = State::Uninit(true);
    let mut out = 0;
    for c in contents.chars() {
        let (new_state, maybe_mult) = process(state, c, handle_conditionals);
        state = new_state;
        if let Some(mult) = maybe_mult {
            out += mult;
        }
    }
    out
}

fn compute_1(contents: &str) -> u64 {
    compute(contents, false)
}

fn compute_2(contents: &str) -> u64 {
    compute(contents, true)
}

pub(crate) struct Day {}

impl Problem for Day {
    fn source_code_file(&self) -> String {
        file!().to_string()
    }
    fn solve1(&self, contents: &str) -> String {
        format!("{}", compute_1(contents))
    }
    fn solve2(&self, contents: &str) -> String {
        format!("{}", compute_2(contents))
    }
    fn expected1(&self) -> String {
        "162813399".to_string()
    }
    fn expected2(&self) -> String {
        "53783319".to_string()
    }
}
