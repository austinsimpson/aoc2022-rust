use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct GenericParseErr;

#[derive(Clone, Copy)]
enum Instruction {
    NoOp,
    Add(i32)
}

impl FromStr for Instruction {
    type Err = GenericParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split_iter = s.split(" ");
        let instruction = split_iter.next().unwrap();
        match instruction {
            "noop" => Ok(Instruction::NoOp),
            "addx" => {
                let add_by = i32::from_str(split_iter.next().unwrap()).unwrap();
                Ok(Instruction::Add(add_by))
            },
            _ => Err(GenericParseErr)
        }
    }
}

struct CPU {
    register: i32,
    current_instruction: Option<Instruction>,
    remaining_cycles_for_current_instruction: i32,
    current_cycle: usize
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            register: 1,
            current_instruction: None,
            remaining_cycles_for_current_instruction: 0,
            current_cycle: 0
        }
    }
}

fn cycles_for_instruction(instruction: &Instruction) -> i32 {
    match instruction {
        Instruction::NoOp => 1,
        Instruction::Add(_) => 2
    }
}

impl CPU {
    fn process_instructions<I>(&mut self, mut instruction_iter: I, cycles_of_interest: &Vec<usize>) -> Vec<i32> 
    where I: Iterator<Item = Instruction> {
        let mut result = vec![];
        self.current_cycle = 0;
        self.remaining_cycles_for_current_instruction = 0;
        self.register = 1;

        loop {
            self.current_cycle += 1;

            //First, we want to fetch the next instruction if we do not have one we're currently executing
            if self.current_instruction.is_none() {
                self.current_instruction = instruction_iter.next();
                match &self.current_instruction {
                    Some(instruction) => { self.remaining_cycles_for_current_instruction = cycles_for_instruction(instruction) },
                    None => break
                }
            }

            //Added for observing the signal strength values outlined in the problem
            if cycles_of_interest.contains(&self.current_cycle) {
                result.push(self.register);
            }            

            self.remaining_cycles_for_current_instruction -= 1;
            if self.remaining_cycles_for_current_instruction == 0 {
                match self.current_instruction.unwrap() {
                    Instruction::NoOp => (),
                    Instruction::Add(val) => self.register += val                    
                }
                self.current_instruction = None;
            }
        }

        result
    }
}

fn main() {
    let instructions = include_str!("input.txt").lines().map(|l| Instruction::from_str(l).unwrap()).collect::<Vec<_>>();

    let mut cpu = CPU::default();

    //Part 1;
    let intervals = vec![20, 60, 100, 140, 180, 220];
    let values = cpu.process_instructions(instructions.clone().into_iter(), &intervals);

    let signal_strength_sum: i32 = intervals.iter().zip(values.iter()).map(|(i, v)| (*i as i32) * (*v)).sum();
    println!("The signal strength is {}", signal_strength_sum);

    //Part 2 
    let crt_intervals = (1..241).collect::<Vec<_>>();
    let register_values = cpu.process_instructions(instructions.clone().into_iter(), &crt_intervals);
    println!("{}", register_values.len());
    let image = crt_intervals.iter().zip(register_values.iter()).map(|(i, v)| { 
        let difference = *v - ((*i - 1) % 40) as i32;
        if i32::abs(difference) <= 1 {
            return '#'
        }
        '.'
    });

    for (pixel_idx, pixel) in image.enumerate() {
        if pixel_idx != 0 && pixel_idx % 40 == 0 {
            print!("\n");
        }
        print!("{}", pixel);
    }
}
