use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TestFunction {
    Lesser,
    Greater,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rule {
    cat: Category,
    test: TestFunction,
    value: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Destination {
    Accept,
    Reject,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WorkflowStep {
    rule: Option<Rule>,
    dest: Destination,
}

type Workflow = Vec<WorkflowStep>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

fn parse_workflow(line: &str) -> (String, Workflow) {
    let mut line_split = line.split('{');
    let name = line_split.next().unwrap();
    let workflow = line_split
        .next()
        .unwrap()
        .strip_suffix('}')
        .unwrap()
        .split(',')
        .map(|rule_str| {
            if rule_str.contains(':') {
                let mut rule_split = rule_str.split(':');
                let mut chars = rule_split.next().unwrap().chars();
                let cat = match chars.next().unwrap() {
                    'x' => Category::X,
                    'm' => Category::M,
                    'a' => Category::A,
                    's' => Category::S,
                    _ => panic!("Unexpected character while processing rule part type"),
                };
                let test = match chars.next().unwrap() {
                    '>' => TestFunction::Greater,
                    '<' => TestFunction::Lesser,
                    _ => panic!("Unexpected character while processing test function type"),
                };
                let value = chars.collect::<String>().parse().unwrap();
                let destination = match rule_split.next().unwrap() {
                    "A" => Destination::Accept,
                    "R" => Destination::Reject,
                    other => Destination::Other(other.to_string()),
                };
                WorkflowStep {
                    rule: Some(Rule { cat, test, value }),
                    dest: destination,
                }
            } else {
                WorkflowStep {
                    rule: None,
                    dest: match rule_str {
                        "A" => Destination::Accept,
                        "R" => Destination::Reject,
                        other => Destination::Other(other.to_string()),
                    },
                }
            }
        })
        .collect();
    (name.to_string(), workflow)
}

fn parse_part(line: &str) -> Part {
    let mut line_split = line
        .strip_prefix('{')
        .unwrap()
        .strip_suffix('}')
        .unwrap()
        .split(',');
    let mut extract_part = |s: String| {
        line_split
            .next()
            .unwrap()
            .strip_prefix(&s)
            .unwrap_or_else(|| panic!("Did not find {s} where expected"))
            .parse::<u64>()
            .unwrap()
    };
    let x = extract_part("x=".to_string());
    let m = extract_part("m=".to_string());
    let a = extract_part("a=".to_string());
    let s = extract_part("s=".to_string());
    Part { x, m, a, s }
}

fn parse_input(contents: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut split = contents.trim().split("\n\n");
    let workflows = split
        .next()
        .unwrap()
        .split('\n')
        .map(parse_workflow)
        .collect();
    let parts = split.next().unwrap().split('\n').map(parse_part).collect();

    (workflows, parts)
}

fn is_accepted(workflows: &HashMap<String, Workflow>, part: &Part) -> bool {
    let mut curr_workflow_steps: VecDeque<&WorkflowStep> = workflows["in"].iter().collect();

    while !curr_workflow_steps.is_empty() {
        let workflow_step = curr_workflow_steps.pop_front().unwrap();
        let workflow_step_matches_part = match workflow_step.rule {
            None => true,
            Some(rule) => {
                let test_function = match rule.test {
                    TestFunction::Lesser => u64::le,
                    TestFunction::Greater => u64::gt,
                };
                test_function(
                    match rule.cat {
                        Category::X => &part.x,
                        Category::M => &part.m,
                        Category::A => &part.a,
                        Category::S => &part.s,
                    },
                    &rule.value,
                )
            }
        };
        if workflow_step_matches_part {
            match &workflow_step.dest {
                Destination::Accept => {
                    return true;
                }
                Destination::Reject => {
                    return false;
                }
                Destination::Other(name) => {
                    curr_workflow_steps.drain(..);
                    curr_workflow_steps.extend(workflows[name].iter())
                }
            };
        }
    }
    panic!("Unreachable. Or is it?");
}

fn compute_1(contents: &str) -> u64 {
    let (workflows, parts) = parse_input(contents);
    let mut summand = 0;
    for part in parts {
        if is_accepted(&workflows, &part) {
            summand += part.x + part.m + part.a + part.s;
        }
    }
    summand
}

const MIN_VAL: u64 = 1;
const MAX_VAL: u64 = 4000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    lo: u64,
    hi: u64,
}

impl Range {
    fn intersect(&self, other: &Self) -> Option<Self> {
        let lo = std::cmp::max(self.lo, other.lo);
        let hi = std::cmp::min(self.hi, other.hi);
        if lo > hi {
            None
        } else {
            Some(Range { lo, hi })
        }
    }

    fn count(&self) -> u64 {
        self.hi - self.lo + 1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PartsRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RuleApplicationResult {
    accepted_ranges: Vec<PartsRange>,
    workflows_to_do: Vec<(String, PartsRange)>,
}

impl PartsRange {
    fn default() -> Self {
        PartsRange {
            x: Range {
                lo: MIN_VAL,
                hi: MAX_VAL,
            },
            m: Range {
                lo: MIN_VAL,
                hi: MAX_VAL,
            },
            a: Range {
                lo: MIN_VAL,
                hi: MAX_VAL,
            },
            s: Range {
                lo: MIN_VAL,
                hi: MAX_VAL,
            },
        }
    }

    fn count(&self) -> u64 {
        self.x.count() * self.m.count() * self.a.count() * self.s.count()
    }

    fn filter(&self, rule: Rule) -> (Option<Self>, Option<Self>) {
        let (matching_intersecter, nonmatching_intersecter) = match rule.test {
            TestFunction::Lesser => (
                Range {
                    lo: MIN_VAL,
                    hi: rule.value - 1,
                },
                Range {
                    lo: rule.value,
                    hi: MAX_VAL,
                },
            ),
            TestFunction::Greater => (
                Range {
                    lo: rule.value + 1,
                    hi: MAX_VAL,
                },
                Range {
                    lo: MIN_VAL,
                    hi: rule.value,
                },
            ),
        };
        macro_rules! process_category {
            ($field:ident) => {
                (
                    if let Some(intersection) = self.$field.intersect(&matching_intersecter) {
                        Some(PartsRange {
                            $field: intersection,
                            ..*self
                        })
                    } else {
                        None
                    },
                    if let Some(intersection) = self.$field.intersect(&nonmatching_intersecter) {
                        Some(PartsRange {
                            $field: intersection,
                            ..*self
                        })
                    } else {
                        None
                    },
                )
            };
        }
        match rule.cat {
            Category::X => process_category!(x),
            Category::M => process_category!(m),
            Category::A => process_category!(a),
            Category::S => process_category!(s),
        }
    }

    fn apply_workflow(&self, workflow: &Workflow) -> RuleApplicationResult {
        let mut accepted_ranges = vec![];
        let mut workflows_to_do = vec![];

        let mut current_range = *self;

        for workflow_step in workflow {
            match workflow_step.rule {
                None => match &workflow_step.dest {
                    Destination::Accept => {
                        accepted_ranges.push(current_range);
                    }
                    Destination::Reject => {}
                    Destination::Other(name) => {
                        workflows_to_do.push((name.clone(), current_range));
                    }
                },
                Some(rule) => {
                    let (maybe_passing_range, maybe_nonpassing_range) = current_range.filter(rule);
                    if let Some(passing_range) = maybe_passing_range {
                        match &workflow_step.dest {
                            Destination::Accept => {
                                accepted_ranges.push(passing_range);
                            }
                            Destination::Reject => {}
                            Destination::Other(name) => {
                                workflows_to_do.push((name.clone(), passing_range));
                            }
                        }
                    }
                    if let Some(nonpassing_range) = maybe_nonpassing_range {
                        current_range = nonpassing_range;
                    } else {
                        break;
                    }
                }
            };
        }
        RuleApplicationResult {
            accepted_ranges,
            workflows_to_do,
        }
    }
}

fn compute_2(contents: &str) -> u64 {
    let workflows = parse_input(contents).0;
    let mut queue: Vec<(String, PartsRange)> = vec![("in".to_string(), PartsRange::default())];
    let mut accepted: Vec<PartsRange> = vec![];

    while let Some((workflow_name, parts_range)) = queue.pop() {
        let workflow = &workflows[&workflow_name];
        let rule_application_results = parts_range.apply_workflow(workflow);
        accepted.extend(rule_application_results.accepted_ranges);
        queue.extend(rule_application_results.workflows_to_do);
    }

    accepted.iter().map(|parts_range| parts_range.count()).sum()
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d19.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(330820, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!(123972546935551, result);
    println!("part 2: {result}");
}
