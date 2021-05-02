use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Result};

pub enum Direction {
    Left,
    Right,
    Stay,
}

pub struct TuringMachine<'a> {
    state: &'a str,
    terminating_states: Vec<&'a str>,
    rules: Vec<Rule<'a>>,
    band: VecDeque<char>,
    head: usize,
    blank: char,
}

pub struct Rule<'a> {
    state: &'a str,
    read: Symbol,
    write: Symbol,
    dir: Direction,
    new_state: &'a str,
}

impl<'a> Rule<'a> {
    pub fn new(state: &'a str, read: char, write: char, dir: Direction, new_state: &'a str) -> Self {
        Self {
            state,
            read,
            write,
            dir,
            new_state,
        }
    }
}

type Symbol = char;

impl<'a> TuringMachine<'a> {
    pub fn new(
        initial_state: &'a str,
        terminating_states: Vec<&'a str>,
        blank: char,
        rules: Vec<Rule<'a>>,
        input: &str,
    ) -> Self {
        Self {
            state: initial_state,
            terminating_states,
            rules,
            band: input.chars().collect::<VecDeque<_>>(),
            head: 0,
            blank,
        }
    }

    pub fn is_done(&self) -> bool {
        self.terminating_states.contains(&self.state)
    }
    pub fn step(&mut self) {
        let field = self.band.get(self.head).unwrap();
        let rule = self
            .rules
            .iter()
            .find(|rule| rule.state == self.state && &rule.read == field)
            .unwrap();
        let field = self.band.get_mut(self.head).unwrap();
        *field = rule.write;
        self.state = rule.new_state;
        match rule.dir {
            Direction::Left => {
                if self.head == 0 {
                    self.band.push_front(self.blank)
                } else {
                    self.head -= 1;
                }
            }
            Direction::Right => {
                if self.head == self.band.len() - 1 {
                    self.band.push_back(self.blank)
                }
                self.head += 1;
            }
            Direction::Stay => {}
        }
    }
    pub fn tape(&self) -> String {
        self.band
            .iter()
            .filter(|x| *x != &'_')
            .fold(String::new(), |acc, val| format!("{}{}", acc, val))
    }
}
impl<'a> Display for TuringMachine<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let band = self
            .band
            .iter()
            .enumerate()
            .map(|(i, c)| {
                if i == self.head {
                    format!("[{}]", c)
                } else {
                    format!(" {} ", c)
                }
            })
            .fold(String::new(), |acc, val| acc + &val);
        write!(f, "{}\t{}", self.state, band)
    }
}
