use shared::Problem;

struct Rgb {
    r: u64,
    g: u64,
    b: u64,
}

struct Game {
    id: u64,
    games: Vec<Rgb>,
}

fn parse_game(game_string: &str) -> Rgb {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    for s in game_string.split(", ") {
        let mut s_split = s.split(' ');
        let num = s_split.next().unwrap().parse().unwrap();
        let color = s_split.next().unwrap();
        if color == "red" {
            r = num;
        } else if color == "green" {
            g = num;
        } else if color == "blue" {
            b = num;
        }
    }
    Rgb { r, g, b }
}

fn parse_line(line: &str) -> Game {
    let mut split_line = line.split(": ");
    let first = split_line.next().unwrap();
    let second = split_line.next().unwrap();
    Game {
        id: first.split(' ').nth(1).unwrap().parse().unwrap(),
        games: second.split("; ").map(parse_game).collect(),
    }
}

fn parse_input(contents: &str) -> Vec<Game> {
    contents
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(parse_line)
        .collect()
}

fn game_plausible(game: &Game, rgb: &Rgb) -> bool {
    !game
        .games
        .iter()
        .any(|g| g.r > rgb.r || g.g > rgb.g || g.b > rgb.b)
}

fn power_min_viable_cubes(game: &Game) -> u64 {
    let r = game.games.iter().map(|g| g.r).max().unwrap();
    let g = game.games.iter().map(|g| g.g).max().unwrap();
    let b = game.games.iter().map(|g| g.b).max().unwrap();
    r * g * b
}

pub(crate) struct Day {}

impl Problem for Day {
    fn source_code_file(&self) -> String {
        file!().to_string()
    }
    fn solve1(&self, contents: &str) -> String {
        let games = parse_input(contents);

        let limit = Rgb {
            r: 12,
            g: 13,
            b: 14,
        };

        let result: u64 = games
            .iter()
            .filter(|g| game_plausible(g, &limit))
            .map(|g| g.id)
            .sum();
        format!("{}", result)
    }
    fn expected1(&self) -> String {
        "2545".to_string()
    }
    fn solve2(&self, contents: &str) -> String {
        let games = parse_input(contents);
        let result: u64 = games.iter().map(power_min_viable_cubes).sum();
        format!("{}", result)
    }
    fn expected2(&self) -> String {
        "78111".to_string()
    }
}
