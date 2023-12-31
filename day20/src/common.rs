use std::{cell::RefCell, collections::HashMap, fmt::Display, str::FromStr};

use anyhow::Error;

type ModuleName = String;

#[derive(Debug, Default, Clone, Copy)]
enum Pulse {
    #[default]
    Low,
    High,
}
impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::High => "high",
            Self::Low => "low",
        };
        write!(f, "{s}")
    }
}

pub struct Puzzle {
    inner: RefCell<HashMap<String, Module>>,
    low_pulses: RefCell<usize>,
    high_pulses: RefCell<usize>,
}
impl Puzzle {
    pub fn run(&self) -> usize {
        for _ in 0..1000 {
            self.run_cycle();
        }
        *self.low_pulses.borrow() * *self.high_pulses.borrow()
    }
    pub fn run_cycle(&self) {
        *self.low_pulses.borrow_mut() += 1; //button -low->broadcast
        self.send_pulse("broadcaster", Pulse::Low);
    }
    fn send_pulse(&self, input: &str, pulse: Pulse) {
        let output_modules = self.inner.borrow().get(input).unwrap().outputs();

        // process pulses
        let new_pulses = output_modules
            .iter()
            .map(|module_name| {
                match pulse {
                    Pulse::Low => {
                        *self.low_pulses.borrow_mut() += 1;
                    }
                    Pulse::High => {
                        *self.high_pulses.borrow_mut() += 1;
                    }
                };
                (
                    module_name,
                    self.inner
                        .borrow_mut()
                        .get_mut(module_name)
                        .and_then(|module| module.process_pulse(input, pulse)),
                )
            })
            .collect::<Vec<_>>();

        // propagate
        for (module_name, pulse) in new_pulses {
            if let Some(pulse) = pulse {
                self.send_pulse(module_name, pulse);
            }
        }
    }
}
impl FromStr for Puzzle {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // first pass to create all the modules
        let mut modules: HashMap<String, Module> = s
            .lines()
            .map(|line| {
                let ll = line.split_once(" -> ").unwrap();
                let module = Module::from_str(ll.0).unwrap();
                let name = ll.0.replace(['%', '&'], "");
                (name, module)
            })
            .collect();

        // second pass to connect the modules them
        s.lines().for_each(|line| {
            let ll = line.split_once(" -> ").unwrap();
            let input = Module::input_name(ll.0);
            let outputs: Vec<String> = ll.1.split(',').map(|s| s.trim().to_string()).collect();

            // if an output is a conjunction, it means the current module should be added to the input list of the conunction
            for output in outputs.iter() {
                if let Some(Module::Conjunction(cj)) = modules.get_mut(output) {
                    cj.inputs.insert(input.clone(), Pulse::default());
                }
            }

            // set the outputs of the module
            if let Some(module) = modules.get_mut(&input) {
                match module {
                    Module::FlipFlop(ff) => {
                        ff.outputs = outputs;
                    }
                    Module::Conjunction(cj) => {
                        cj.output = outputs;
                    }
                    Module::Broadcast(bd) => {
                        bd.outputs = outputs;
                    }
                }
            }
        });

        Ok(Self {
            inner: RefCell::new(modules),
            low_pulses: RefCell::new(0),
            high_pulses: RefCell::new(0),
        })
    }
}
impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

#[derive(Debug)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcast(Broadcast),
}
impl FromStr for Module {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('%') {
            Ok(Self::FlipFlop(FlipFlop::default()))
        } else if s.starts_with('&') {
            Ok(Self::Conjunction(Conjunction::default()))
        } else {
            Ok(Self::Broadcast(Broadcast::default()))
        }
    }
}
impl Module {
    fn input_name(s: &str) -> String {
        s.replace(['%', '&'], "")
    }
    fn process_pulse(&mut self, input: &str, pulse: Pulse) -> Option<Pulse> {
        match self {
            Self::FlipFlop(ff) => ff.process_pulse(pulse),
            Self::Conjunction(cj) => Some(cj.process_pulse(input, pulse)),
            Self::Broadcast(bd) => Some(bd.process_pulse()),
        }
    }
    fn outputs(&self) -> Vec<String> {
        match self {
            Self::FlipFlop(ff) => ff.outputs.clone(),
            Self::Conjunction(cj) => cj.output.clone(),
            Self::Broadcast(bd) => bd.outputs.clone(),
        }
    }
}

#[derive(Debug, Default)]
struct FlipFlop {
    on: bool,
    outputs: Vec<ModuleName>,
}
impl FlipFlop {
    fn process_pulse(&mut self, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            // Pulse::High => Some(Pulse::High),
            Pulse::High => None,
            Pulse::Low => {
                // If it was on, it turns off and sends a low pulse.
                if self.on {
                    self.on = false;
                    Some(Pulse::Low)
                }
                // If it was off, it turns on and sends a high pulse.
                else {
                    self.on = true;
                    Some(Pulse::High)
                }
            }
        }
    }
}

#[derive(Debug, Default)]
struct Conjunction {
    inputs: HashMap<ModuleName, Pulse>,
    output: Vec<ModuleName>,
}
impl Conjunction {
    fn process_pulse(&mut self, input: &str, pulse: Pulse) -> Pulse {
        if let Some(i) = self.inputs.get_mut(input) {
            *i = pulse;
        }
        // if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
        if self.inputs.values().all(|p| match p {
            Pulse::High => true,
            Pulse::Low => false,
        }) {
            Pulse::Low
        } else {
            Pulse::High
        }
    }
}

#[derive(Debug, Default)]
struct Broadcast {
    outputs: Vec<ModuleName>,
}
impl Broadcast {
    fn process_pulse(&self) -> Pulse {
        Pulse::Low
    }
}
