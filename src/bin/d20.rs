use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Hi,
    Lo,
}

type ModuleId = usize;

trait ModuleInterface {
    fn process_pulse(&mut self, sender_id: ModuleId, pulse: Pulse) -> Option<Pulse>;

    fn id(&self) -> ModuleId;

    fn destinations(&self) -> &Vec<ModuleId>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FlipFlop {
    id: ModuleId,
    name: String,
    is_on: bool,
    destinations: Vec<ModuleId>,
}

impl FlipFlop {
    fn new(id: ModuleId, name: String, destinations: Vec<ModuleId>) -> Self {
        FlipFlop {
            id,
            name,
            is_on: false,
            destinations,
        }
    }
}

impl ModuleInterface for FlipFlop {
    fn process_pulse(&mut self, _sender_id: ModuleId, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::Hi => None,
            Pulse::Lo => {
                self.is_on = !self.is_on;
                if self.is_on {
                    Some(Pulse::Hi)
                } else {
                    Some(Pulse::Lo)
                }
            }
        }
    }

    fn id(&self) -> ModuleId {
        self.id
    }

    fn destinations(&self) -> &Vec<ModuleId> {
        &self.destinations
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Conjunction {
    id: ModuleId,
    name: String,
    connected_inputs: HashMap<ModuleId, Pulse>,
    destinations: Vec<ModuleId>,
}

impl Conjunction {
    fn new(id: ModuleId, name: String, destinations: Vec<ModuleId>) -> Self {
        Conjunction {
            id,
            name,
            connected_inputs: HashMap::new(),
            destinations,
        }
    }
}

impl ModuleInterface for Conjunction {
    fn process_pulse(&mut self, sender_id: ModuleId, pulse: Pulse) -> Option<Pulse> {
        self.connected_inputs.insert(sender_id, pulse);

        if self.connected_inputs.values().all(|p| p == &Pulse::Hi) {
            Some(Pulse::Lo)
        } else {
            Some(Pulse::Hi)
        }
    }

    fn id(&self) -> ModuleId {
        self.id
    }

    fn destinations(&self) -> &Vec<ModuleId> {
        &self.destinations
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Broadcast {
    id: ModuleId,
    name: String,
    destinations: Vec<ModuleId>,
}

impl Broadcast {
    fn new(id: ModuleId, name: String, destinations: Vec<ModuleId>) -> Self {
        Broadcast {
            id,
            name,
            destinations,
        }
    }
}

impl ModuleInterface for Broadcast {
    fn process_pulse(&mut self, _sender_id: ModuleId, pulse: Pulse) -> Option<Pulse> {
        Some(pulse)
    }

    fn id(&self) -> ModuleId {
        self.id
    }

    fn destinations(&self) -> &Vec<ModuleId> {
        &self.destinations
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleKind {
    FlipFlop,
    Conjunction,
    Broadcast,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcast(Broadcast),
}

impl Module {
    fn new(
        id: ModuleId,
        module_name: String,
        destinations: Vec<ModuleId>,
        kind: ModuleKind,
    ) -> Self {
        match kind {
            ModuleKind::FlipFlop => Module::FlipFlop(FlipFlop::new(id, module_name, destinations)),
            ModuleKind::Conjunction => {
                Module::Conjunction(Conjunction::new(id, module_name, destinations))
            }
            ModuleKind::Broadcast => {
                Module::Broadcast(Broadcast::new(id, module_name, destinations))
            }
        }
    }

    fn name(&self) -> &String {
        match self {
            Module::FlipFlop(flip_flop) => &flip_flop.name,
            Module::Conjunction(conjunction) => &conjunction.name,
            Module::Broadcast(broadcast) => &broadcast.name,
        }
    }
}

impl ModuleInterface for Module {
    fn process_pulse(&mut self, sender_id: ModuleId, pulse: Pulse) -> Option<Pulse> {
        match self {
            Module::FlipFlop(flip_flop) => flip_flop.process_pulse(sender_id, pulse),
            Module::Conjunction(conjunction) => conjunction.process_pulse(sender_id, pulse),
            Module::Broadcast(broadcast) => broadcast.process_pulse(sender_id, pulse),
        }
    }

    fn id(&self) -> ModuleId {
        match self {
            Module::FlipFlop(flip_flop) => flip_flop.id(),
            Module::Conjunction(conjunction) => conjunction.id(),
            Module::Broadcast(broadcast) => broadcast.id(),
        }
    }

    fn destinations(&self) -> &Vec<ModuleId> {
        match self {
            Module::FlipFlop(flip_flop) => flip_flop.destinations(),
            Module::Conjunction(conjunction) => conjunction.destinations(),
            Module::Broadcast(broadcast) => broadcast.destinations(),
        }
    }
}

fn parse_input(contents: &String) -> (ModuleId, HashMap<ModuleId, Module>) {
    let mut id: usize = 0;
    let mut name_to_id: HashMap<String, ModuleId> = HashMap::new();
    let mut id_to_module: HashMap<ModuleId, Module> = HashMap::new();
    let mut broadcast_id = 0;
    for line in contents.trim().split('\n') {
        let (mut split, module_kind) = if line.starts_with('%') {
            (
                line.strip_prefix('%').unwrap().split(" -> "),
                ModuleKind::FlipFlop,
            )
        } else if line.starts_with('&') {
            (
                line.strip_prefix('&').unwrap().split(" -> "),
                ModuleKind::Conjunction,
            )
        } else {
            (line.split(" -> "), ModuleKind::Broadcast)
        };

        let module_name = split.next().unwrap().to_string();
        let destination_names: Vec<String> = split
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.to_string())
            .collect();

        // Create IDs for any newly encountered names
        for name in destination_names
            .iter()
            .chain(vec![module_name.clone()].iter())
        {
            name_to_id.entry(name.clone()).or_insert(id);
            id += 1;
        }

        let module_id = name_to_id[&module_name];
        let destination_ids: Vec<ModuleId> = destination_names
            .into_iter()
            .map(|dest_name| name_to_id[&dest_name])
            .collect();

        match module_kind {
            ModuleKind::FlipFlop => id_to_module.insert(
                module_id,
                Module::new(
                    module_id,
                    module_name,
                    destination_ids.clone(),
                    ModuleKind::FlipFlop,
                ),
            ),
            ModuleKind::Conjunction => id_to_module.insert(
                module_id,
                Module::new(
                    module_id,
                    module_name,
                    destination_ids.clone(),
                    ModuleKind::Conjunction,
                ),
            ),
            ModuleKind::Broadcast => {
                broadcast_id = module_id;
                id_to_module.insert(
                    module_id,
                    Module::new(
                        module_id,
                        module_name,
                        destination_ids.clone(),
                        ModuleKind::Broadcast,
                    ),
                )
            }
        };
    }
    // Register all inputs for every conjunction module
    let mut incoming_connections: HashMap<ModuleId, Vec<ModuleId>> = HashMap::new();
    for source_module in id_to_module.values() {
        for destination_id in source_module.destinations() {
            incoming_connections
                .entry(*destination_id)
                .or_insert(Vec::new())
                .push(source_module.id());
        }
    }
    for possible_conjunction in id_to_module.values_mut() {
        if let Module::Conjunction(ref mut conjunction) = possible_conjunction {
            for incoming_module_id in incoming_connections[&conjunction.id()].iter() {
                conjunction
                    .connected_inputs
                    .entry(*incoming_module_id)
                    .or_insert(Pulse::Lo);
            }
        }
    }
    (broadcast_id, id_to_module)
}

fn push_button(broadcast_id: usize, modules: &mut HashMap<ModuleId, Module>) -> (u64, u64) {
    let mut lo_count = 0;
    let mut hi_count = 0;
    // (source_id, dest_id, pulse)
    let mut pulses: VecDeque<(ModuleId, ModuleId, Pulse)> = VecDeque::new();
    pulses.push_back((usize::MAX, broadcast_id, Pulse::Lo));
    lo_count += 1;
    let mut id_to_name: HashMap<ModuleId, String> = HashMap::new();
    for (id, module) in modules.iter() {
        id_to_name.insert(*id, module.name().clone());
    }
    while pulses.len() > 0 {
        let (source_id, destination_id, pulse) = pulses.pop_front().unwrap();
        modules.entry(destination_id).and_modify(|module| {
            if let Some(new_pulse) = module.process_pulse(source_id, pulse) {
                for new_destination_id in module.destinations().iter() {
                    match new_pulse {
                        Pulse::Hi => hi_count += 1,
                        Pulse::Lo => lo_count += 1,
                    };
                    pulses.push_back((destination_id, *new_destination_id, new_pulse))
                }
            }
        });
    }
    (lo_count, hi_count)
}

fn compute_1(contents: &String) -> u64 {
    let (broadcast_id, mut modules) = parse_input(contents);
    let mut lo_count = 0;
    let mut hi_count = 0;
    for _ in 0..1000 {
        let (curr_lo, curr_hi) = push_button(broadcast_id, &mut modules);
        lo_count += curr_lo;
        hi_count += curr_hi;
    }
    lo_count * hi_count
}

// fn compute_2(contents: &String) -> u64 {}

fn main() {
    let contents =
        fs::read_to_string("inputs/d20.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    // assert_eq!(670984704, result);
    println!("part 1: {result}");

    // let result = compute_2(&contents);
    // assert_eq!(123972546935551, result);
    // println!("part 2: {result}");
}