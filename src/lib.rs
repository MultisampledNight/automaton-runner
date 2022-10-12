mod input;
mod output;
mod parse;
mod run;

use std::{collections::BTreeMap, error::Error, fs};

pub type Map<K, V> = BTreeMap<K, V>;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id(String);

#[derive(Copy, Clone, Debug)]
pub enum Role {
    None,
    End,
    Catch,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub transitions: Map<char, Id>,
    pub role: Role,
}

#[derive(Clone, Debug)]
pub struct Automaton {
    pub nodes: Map<Id, Node>,
    pub start_at: Id,
}

#[derive(Clone, Debug)]
pub struct ExecutedAutomaton {
    pub conclusion: Conclusion,
    pub path_taken: Vec<Id>,
}

#[derive(Clone, Debug)]
pub enum Conclusion {
    Accepted,
    NotAtEndnode,
    AtCatchnode,
    UnknownTarget { from: Id, through: char, target: Id },
    UnknownChar { at: Id, input: char },
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let args = input::args();
    let source = fs::read_to_string(args.automaton_path)?;

    let machine: Automaton = source.parse()?;
    let execution_result = machine.run(args.input.chars());

    println!("{}", execution_result);
    Ok(())
}
