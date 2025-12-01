struct ComputedValues {
    expected1: String,
    out1: String,
    expected2: String,
    out2: String,
}

pub trait Problem {
    /// Should be implemented with just `file!().to_string()`
    fn source_code_file(&self) -> String;

    /// No need to override the default implementation.
    fn input(&self) -> String {
        let source_code_file = self.source_code_file();
        let input_file_path = std::path::Path::new(&source_code_file);
        let input_year = input_file_path
            .parent() // root/crates/yYYYY/src
            .unwrap()
            .parent() // root/crates/yYYYY
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .split_off(1); // take off the preceding `y`
        let input_file_name = input_file_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .replace(".rs", ".txt");
        format!("inputs/{input_year}/{input_file_name}")
    }

    /// The solution to part 1, returned as a string.
    fn solve1(&self, contents: &str) -> String;
    /// The solution to part 2, returned as a string.
    fn solve2(&self, _: &str) -> String {
        "TODO".to_string()
    }

    /// The expected value for part 1, used to prevent regressions when doing cleanup/refactoring.
    fn expected1(&self) -> String {
        "TODO".to_string()
    }
    /// The expected value for part 2, used to prevent regressions when doing cleanup/refactoring.
    fn expected2(&self) -> String {
        "TODO".to_string()
    }

    #[allow(private_interfaces)]
    fn get_all_computed_values(&self) -> ComputedValues {
        let input_file = self.input();
        let contents = std::fs::read_to_string(input_file.clone())
            .expect(&format!("Trouble reading file {}", input_file));

        let expected1 = self.expected1();
        let out1 = self.solve1(&contents);

        let expected2 = self.expected2();
        let out2 = self.solve2(&contents);

        ComputedValues {
            expected1,
            out1,
            expected2,
            out2,
        }
    }

    /// Run parts 1 and 2 in an interactive (i.e. printing) way.
    fn run(&self) {
        let ComputedValues {
            expected1,
            out1,
            expected2,
            out2,
        } = self.get_all_computed_values();

        if expected1 != out1 {
            eprintln!("WARNING: Actual part 1 output {out1} != expected output {expected1}");
        }
        println!("part 1: {out1}");

        if expected2 != out2 {
            eprintln!("WARNING: Actual part 2 output {out2} != expected output {expected2}");
        }
        if out2 != "TODO".to_string() {
            println!("part 2: {out2}");
        }
    }

    /// Test that parts 1 and 2 both output their expected values.
    /// Will panic if they do not.
    fn verify(&self) {
        let ComputedValues {
            expected1,
            out1,
            expected2,
            out2,
        } = self.get_all_computed_values();

        if out1 != expected1 {
            panic!(
                "Error in {} part 1: {out1} != {expected1}",
                self.source_code_file()
            )
        }

        if out2 != expected2 {
            panic!(
                "Error in {} part 2: {out2} != {expected2}",
                self.source_code_file()
            )
        }
    }
}
