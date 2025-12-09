use std::{fs, io::Write, path::Path};

use shared::Year;
use y2023;
use y2024;
use y2025;

fn make_new(year: u32, day: u8) -> Result<(), String> {
    // Copy template.rs to the correct place
    let src = Path::new("template.rs");
    let dest_str = format!("crates/y{}/src/d{:0>2}.rs", year, day);
    let dest = Path::new(&dest_str);
    if dest.exists() {
        return Err(format!("Destination {dest:?} already exists"));
    }
    fs::copy(src, dest).map_err(|e| format!("Problem copying {src:?} to {dest:?}: {e}"))?;

    // Update the mod.rs file in the year
    let mod_file_path_str = format!("crates/y{}/src/lib.rs", year);
    let mod_file_path = Path::new(&mod_file_path_str);
    let mut mod_contents = fs::read_to_string(mod_file_path)
        .map_err(|e| format!("Problem reading contents of {mod_file_path:?}: {e}"))?;
    mod_contents =
        mod_contents.replace(&format!("// mod d{day:0>2};"), &format!("mod d{day:0>2};"));
    mod_contents = mod_contents.replace(
        &format!("// problems.insert({day}, Box::new(d{day:0>2}::Day {{}}));"),
        &format!("problems.insert({day}, Box::new(d{day:0>2}::Day {{}}));"),
    );
    fs::write(mod_file_path, mod_contents)
        .map_err(|e| format!("Problem writing updates to {mod_file_path:?}: {e}"))?;

    // Create an empty input file
    let input_text_file_path_str = format!("inputs/{year}/d{day:0>2}.txt");
    let input_text_file_path = Path::new(&input_text_file_path_str);
    fs::File::create(input_text_file_path)
        .map_err(|e| format!("Problem creating input file {input_text_file_path:?}: {e}"))?;
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1) == Some(&"verify".to_string()) {
        verify(args);
    } else if args.get(1) == Some(&"new".to_string()) {
        new(args);
    } else if args.len() != 3 {
        usage(args);
    } else {
        run_specific_day(args);
    }
}

fn usage(args: Vec<String>) {
    eprintln!("Usage:");
    eprintln!("  {} verify [<year>]", args[0]);
    eprintln!("  {} <year> <day>", args[0]);
    eprintln!("  {} new <year> <day>", args[0]);
}

fn run_specific_day(args: Vec<String>) {
    let year = args[1]
        .parse::<u16>()
        .expect(&format!("Could not parse {} into a year", args[1]));
    let day = args[2]
        .parse::<u8>()
        .expect(&format!("Could not parse {} into a day", args[2]));

    let problems = match year {
        2023 => y2023::Year {}.problems(),
        2024 => y2024::Year {}.problems(),
        2025 => y2025::Year {}.problems(),
        _ => panic!("Unsupported year {}", year),
    };

    problems
        .get(&day)
        .expect(&format!("Unsupported day {} for year {}", day, year))
        .run(false); // TODO: Add show_times as a CLI param
}

fn new(args: Vec<String>) {
    if args.len() == 4 {
        if let Err(e) = make_new(
            args[2].parse().expect("Trouble parsing year"),
            args[3].parse().expect("Trouble parsing day"),
        ) {
            println!("Error creating new day: {e}");
        }
    } else {
        usage(args);
    }
}

fn verify(args: Vec<String>) {
    let requested_year = args
        .get(2)
        .map(|s| s.parse::<u32>().expect("Unable to parse {s} as year"));
    if args.len() <= 3 {
        let mut problem_runtimes = Vec::new();
        let mut years: Vec<Box<dyn Year>> = vec![
            Box::new(y2023::Year {}),
            Box::new(y2024::Year {}),
            Box::new(y2025::Year {}),
        ];
        years = years
            .into_iter()
            .filter(|year| requested_year.is_none() || Some(year.year()) == requested_year)
            .collect();
        if years.is_empty() {
            eprintln!("Unknown year {requested_year:?} was requested");
            return;
        }
        let mut failed = vec![];
        println!("                       1 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2");
        println!("     1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5");
        for year in years {
            print!("{} ", year.year());
            let problems = year.problems();
            let mut days: Vec<u8> = problems.keys().map(|d| *d).collect();
            days.sort();
            for day in days {
                let problem = problems.get(&day).unwrap();
                // Time the test
                let (result, dur1, dur2) = problem.verify();
                problem_runtimes.push((dur1 + dur2, dur1, dur2, (year.year(), day)));
                if result {
                    print!("| ");
                } else {
                    print!("X ");
                    failed.push((year.year(), day));
                }
                std::io::stdout().flush().unwrap();
            }
            println!();
        }
        println!("\nSlowest runtimes:");
        problem_runtimes.sort();
        for n in 0..5 {
            if let Some((duration, dur1, dur2, (year, day))) = problem_runtimes.iter().nth_back(n) {
                println!(
                    "y{year} d{day:0>2} took {:.3}s to run ({:.4}s + {:.4}s).",
                    duration.as_secs_f32(),
                    dur1.as_secs_f32(),
                    dur2.as_secs_f32()
                );
            }
        }
        if !failed.is_empty() {
            print!("{} problems failed verification! ", failed.len());
            print!(
                "{}",
                failed
                    .iter()
                    .take(5)
                    .map(|(year, day)| format!("y{year} d{day}"))
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            if failed.len() > 5 {
                print!(", ...");
            }
            println!("");
        }
    } else {
        usage(args);
    }
}
