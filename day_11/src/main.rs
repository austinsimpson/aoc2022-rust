use std::{str::{Lines, FromStr}, vec, collections::VecDeque};

use itertools::{Itertools, IntoChunks, Chunk, Chunks};

#[derive(Debug)]
enum Symbol {
    Constant(usize),
    Variable
}

impl Symbol {
    fn resolve(&self, value: Option<usize>) -> usize {
        match self {
            Self::Constant(val) => *val,
            Self::Variable => value.unwrap()
        }
    }
}

impl FromStr for Symbol {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_size_result = usize::from_str(s);
        match parse_size_result {
            Ok(v) => Ok(Self::Constant(v)),
            Err(_) => Ok(Self::Variable)
        }
    }

}

#[derive(Debug)]
enum Op {
    Add,
    Multiply,
    Divide
}

impl Op {
    fn eval(&self, lhs: &Symbol, rhs: &Symbol, variable_value: usize) -> usize {
        let lhs_value = lhs.resolve(Some(variable_value));
        let rhs_value = rhs.resolve(Some(variable_value));

        match self {
            Op::Add => lhs_value + rhs_value, 
            Op::Multiply => lhs_value * rhs_value,
            Op::Divide => lhs_value / rhs_value
        }
    }
}

impl FromStr for Op {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Multiply),
            _ => Ok(Self::Add)
        } 
    }
}

#[derive(Debug)]
enum Test {
    Divisible
}

impl Test {
    fn eval(&self, lhs: &Symbol, rhs: &Symbol, variable_value: usize) -> bool {
        let lhs_value = lhs.resolve(Some(variable_value));
        let rhs_value = rhs.resolve(Some(variable_value));

        match self {
            Self::Divisible => lhs_value % rhs_value == 0
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    items_inspected: usize,
    op: Op,
    lhs: Symbol,
    rhs: Symbol,
    compare: Test,
    compare_numerator: Symbol,
    compare_denominator: Symbol,
    send_to: (usize, usize),
}

impl From<Chunk<'_, Lines<'_>>> for Monkey {
    fn from(mut v: Chunk<'_, Lines<'_>>) -> Self {
        v.next(); //Unused monkey name line
        
        //Starting Items
        let starting_items_line = v.next().unwrap();
        let mut starting_items_split_by_colon = starting_items_line.split(": ");
        starting_items_split_by_colon.next();
        let starting_items = starting_items_split_by_colon
            .next()
            .unwrap()
            .split(", ")
            .map(|v| usize::from_str_radix(v, 10).unwrap())
            .collect::<VecDeque<_>>();

        //Operation
        let operation_line = v.next().unwrap();
        let mut operation_line_split_by_colon = operation_line.split(": new = ");
        operation_line_split_by_colon.next();
        let operation_str = operation_line_split_by_colon.next().unwrap();
        let mut operation_str_iter = operation_str.split(" ");
        let lhs = Symbol::from_str(operation_str_iter.next().unwrap()).unwrap();
        let op = Op::from_str(operation_str_iter.next().unwrap()).unwrap();
        let rhs = Symbol::from_str(operation_str_iter.next().unwrap()).unwrap();

        //Test
        let test_line = v.next().unwrap();
        let mut test_line_split = test_line.split("Test: divisible by ");
        test_line_split.next();
        let divisor_value = usize::from_str(test_line_split.next().unwrap()).unwrap();

        let true_line = v.next().unwrap();
        let mut true_line_split = true_line.split("If true: throw to monkey ");
        true_line_split.next();
        let true_value = usize::from_str(true_line_split.next().unwrap()).unwrap();

        let false_line = v.next().unwrap();
        let mut false_line_split = false_line.split("If false: throw to monkey ");
        false_line_split.next();
        let false_value = usize::from_str(false_line_split.next().unwrap()).unwrap();
       

        Monkey { 
            items: starting_items, 
            items_inspected: 0,
            op: op,
            lhs: lhs,
            rhs: rhs,
            compare: Test::Divisible,
            compare_numerator: Symbol::Variable,
            compare_denominator: Symbol::Constant(divisor_value),
            send_to: (true_value, false_value)
        }
    }
}

impl Monkey {
    fn inspect_next_item(&mut self) -> Option<(usize, usize)> {
        let item = self.items.pop_front();
        match item {
            None => None,
            Some(v) => {
                self.items_inspected += 1;
                let new_worry_level = self.op.eval(&self.lhs, &self.rhs, v);
                let compare_result = self.compare.eval(&self.compare_numerator, &self.compare_denominator, new_worry_level);
                let new_monkey_index = match compare_result {
                    true => self.send_to.0,
                    false => self.send_to.1
                };

                Some((new_worry_level, new_monkey_index))
            }
        }
    }

    fn receive_new_item(&mut self, value: usize) {
        self.items.push_back(value)
    }
}

fn main() {
    let lines = include_str!("input.txt").lines();
    let chunks = lines.into_iter().chunks(7);
    let mut monkeys = chunks.into_iter().map(Monkey::from).collect::<Vec<_>>();

    let divisor_product: usize = monkeys.iter().map(|m| m.compare_denominator.resolve(Some(0))).product();

    for _ in 0..10000 {
        for current_monkey_index in 0..monkeys.len() {
            let mut results: Vec<(usize, usize)> = Vec::new();
            {
                let monkey = &mut monkeys[current_monkey_index];
                while let Some(res) = monkey.inspect_next_item() {
                    results.push(res);
                }
            }

            for result in results.into_iter() {
                monkeys[result.1].receive_new_item(result.0 % divisor_product)
            }

        }
    }
    
    monkeys.sort_by(|first, second| second.items_inspected.cmp(&first.items_inspected));
    let monkey_business: usize = monkeys.iter().take(2).map(|m| m.items_inspected).product();
    println!("{monkey_business}");


}
