use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

type Id = [char; 3];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Gate {
    in1: Id,
    in2: Id,
    out: Id,
    op: Operation,
}

fn parse_input(contents: &str) -> (HashMap<Id, Option<bool>>, VecDeque<Gate>, Vec<Id>) {
    let mut wires: HashMap<Id, Option<bool>> = HashMap::new();
    let mut gates: VecDeque<Gate> = VecDeque::new();
    let mut output_wires: Vec<Id> = Vec::new();
    let (initial_wire_strs, gate_strs) = contents.trim().split_once("\n\n").unwrap();
    initial_wire_strs.split('\n').for_each(|wire_str| {
        let (id, value) = wire_str.split_once(": ").unwrap();
        wires.insert(
            id.chars().collect::<Vec<char>>().try_into().unwrap(),
            Some(value == "1"),
        );
    });
    // println!("{wires:?}");
    gate_strs.split('\n').for_each(|gate_str| {
        let words: [&str; 5] = gate_str
            .split(' ')
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();
        let in1: Id = words[0].chars().collect::<Vec<char>>().try_into().unwrap();
        let in2: Id = words[2].chars().collect::<Vec<char>>().try_into().unwrap();
        let out: Id = words[4].chars().collect::<Vec<char>>().try_into().unwrap();

        for id in [in1, in2, out].iter() {
            wires.entry(*id).or_insert(None);
        }

        if out[0] == 'z' {
            output_wires.push(out);
        }

        let op = if words[1] == "AND" {
            Operation::And
        } else if words[1] == "OR" {
            Operation::Or
        } else {
            Operation::Xor
        };

        gates.push_back(Gate { in1, in2, out, op });
    });

    output_wires.sort();

    (wires, gates, output_wires)
}

fn run_gates(
    wires: &mut HashMap<Id, Option<bool>>,
    gates: &mut VecDeque<Gate>,
    output_wires: &[Id],
) -> u64 {
    while !gates.is_empty() {
        let gate = gates.pop_front().unwrap();
        let mut should_put_back = true;
        if let Some(wire1) = wires.get(&gate.in1).unwrap() {
            if let Some(wire2) = wires.get(&gate.in2).unwrap() {
                should_put_back = false;
                match gate.op {
                    Operation::And => wires.insert(gate.out, Some(wire1 & wire2)),
                    Operation::Or => wires.insert(gate.out, Some(wire1 | wire2)),
                    Operation::Xor => wires.insert(gate.out, Some(wire1 ^ wire2)),
                };
            }
        }
        if should_put_back {
            gates.push_back(gate);
        }
    }
    let mut n = 0;
    for id in output_wires.iter().rev() {
        n <<= 1;
        n += wires.get(id).unwrap().unwrap() as u64;
    }
    n
}

fn compute_1(contents: &str) -> u64 {
    let (mut wires, mut gates, output_wires) = parse_input(contents);
    run_gates(&mut wires, &mut gates, &output_wires)
}

fn compute_2(contents: &str) -> String {
    let (_, gates, output_wires) = parse_input(contents);

    // Check for consistency against a ripple carry adder.
    // https://en.wikipedia.org/wiki/Adder_(electronics)#Ripple-carry_adder
    // Ripple Carry Adders are constructed from one Half Adder
    // and many Full Adders (and the final set of logic at the end)

    let mut bad_gates: HashSet<&Gate> = HashSet::new();
    for gate in &gates {
        let is_output_bit = gate.out[0] == 'z';
        let is_half_adder = gate.in1 == ['x', '0', '0'] || gate.in2 == ['x', '0', '0'];
        let takes_inputs = gate.in1[0] == 'x' || gate.in1[0] == 'y';

        // Every output value except the last should result from an XOR
        if is_output_bit
            && (&gate.out != output_wires.last().unwrap())
            && (gate.op != Operation::Xor)
        {
            bad_gates.insert(gate);
        }

        // The first (half) adder should be constructed out of AND and XOR
        if is_half_adder && (gate.op == Operation::Or) {
            bad_gates.insert(gate);
        }

        if gate.op == Operation::Xor {
            // Any XOR must be either connected to two inputs or an output
            if !(takes_inputs || is_output_bit) {
                bad_gates.insert(gate);
            }

            // Any XOR only feeds into AND and XOR
            for gate2 in &gates {
                if (gate.out == gate2.in1 || gate.out == gate2.in2) && gate2.op == Operation::Or {
                    bad_gates.insert(gate);
                }
            }
        }

        // Any AND must only feed into OR (for the full adders)
        if gate.op == Operation::And && !is_half_adder {
            for gate2 in &gates {
                if (gate.out == gate2.in1 || gate.out == gate2.in2) && gate2.op != Operation::Or {
                    bad_gates.insert(gate);
                }
            }
        }
    }

    let mut out: Vec<String> = bad_gates
        .iter()
        .map(|g| format!("{}{}{}", g.out[0], g.out[1], g.out[2]))
        .collect();
    out.sort();
    out.join(",")
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d24.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(53190357879014, result);
    println!("part 1: {result}");

    let result = compute_2(&contents);
    assert_eq!("bks,hnd,nrn,tdv,tjp,z09,z16,z23", result);
    println!("part 2: {result}");
}
