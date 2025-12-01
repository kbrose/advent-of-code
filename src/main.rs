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
        verify(args)
    } else if args.get(1) == Some(&"new".to_string()) {
        new(args)
    } else if args.len() != 3 {
        eprintln!("Usage:");
        eprintln!("  {} test", args[0]);
        eprintln!("  {} <year> <day>", args[0]);
        eprintln!("  {} new <year> <day>", args[0]);
        std::process::exit(1);
    } else {
        run_specific_day(args);
    }
}

fn usage(args: Vec<String>) {
    eprintln!("Usage:");
    eprintln!("  {} test", args[0]);
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
        .run();
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
    if args.len() == 2 {
        println!("                       1 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2");
        println!("     1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5");
        let mut problem_runtimes = Vec::new();
        let years: Vec<Box<dyn Year>> = vec![
            Box::new(y2023::Year {}),
            Box::new(y2024::Year {}),
            Box::new(y2025::Year {}),
        ];
        for year in years {
            print!("{} ", year.year());
            for (day, problem) in year.problems() {
                // Time the test
                let start = std::time::Instant::now();
                problem.verify();
                let duration = start.elapsed();
                problem_runtimes.push((duration, (2023, day)));
                print!("| ");
                std::io::stdout().flush().unwrap();
            }
            println!();
        }
        println!("\nSlowest runtimes:");
        problem_runtimes.sort();
        for n in 0..5 {
            let (duration, (year, day)) = problem_runtimes.iter().nth_back(n).unwrap();
            println!(
                "y{year} d{day:0>2} took {:.3}s to run.",
                duration.as_secs_f32()
            );
        }
    } else {
        usage(args);
    }
}
