pub mod turing_machine {
    use std::{char, time::Instant};

    pub struct TuringMachine {
        pub tape: Vec<char>,
        pub ruleset: Vec<Rule>,
        pub state: String,
        pub time_limit: u64,
        pub rw_head: u64,
    }

    pub struct Rule {
        pub state: String,
        pub instructions: Vec<Instruction>,
        pub next_state: String,
    }

    pub struct Instruction {
        pub symbol: char,
        pub action: Action,
    }
    pub enum Action {
        Write(char),
        Move(MoveHead),
    }

    pub enum MoveHead {
        Left,
        Right,
        None,
    }

    impl TuringMachine {
        pub fn new(
            ruleset: Vec<Rule>,
            state: String,
            time_limit: u64,
        ) -> TuringMachine {
            TuringMachine {
                tape: Vec::new(),
                ruleset,
                state,
                time_limit,
                rw_head: 0,
            }
        }

        pub fn execute(mut self) {
            let start_time: Instant = Instant::now();
            let mut current_time: u64 = start_time.elapsed().as_secs();

            while current_time < self.time_limit {
                let current_state: &String = &self.state.clone();
                let current_symbol: &char = &self.tape[self.rw_head as usize].clone();

                for rule in &self.ruleset {
                    if &rule.state == current_state {
                        for instr in &rule.instructions {
                            if &instr.symbol == current_symbol {
                                match &instr.action {
                                    Action::Write(symbol) => {
                                        self.tape[self.rw_head as usize] = *symbol;
                                    }
                                    Action::Move(mv) => match mv {
                                        MoveHead::Left => {
                                            if self.rw_head > 0 {
                                                self.rw_head -= 1;
                                            } else {
                                                self.tape.insert(0, ' ');
                                            }
                                        }
                                        MoveHead::Right => {
                                            if self.rw_head as usize >= self.tape.len() - 1 {
                                                self.tape.push(' ');
                                            }
                                            self.rw_head += 1;
                                        }
                                        MoveHead::None => {}
                                    },
                                }
                            }
                        }
                    }
                }
                current_time = start_time.elapsed().as_secs();
            }
        }
    }

    impl Rule {
        pub fn new(state: String, instructions: Vec<Instruction>, next_state: String) -> Rule {
            Rule {
                state,
                instructions,
                next_state,
            }
        }

        pub fn quintuple(state: String, symbol: char, action: Action, next_state: String) -> Rule {
            Rule {
                state,
                instructions: vec![Instruction { symbol, action }],
                next_state,
            }
        }
    }

    impl Instruction {
        pub fn new(symbol: char, action: Action) -> Instruction {
            Instruction { symbol, action }
        }
    }

    impl Action {
        pub fn move_head(dir: char) -> Action {
            match dir {
                'L' => Action::Move(MoveHead::Left),
                'R' => Action::Move(MoveHead::Right),
                _ => Action::Move(MoveHead::None),
            }
        }

        pub fn write(symbol: char) -> Action {
            Action::Write(symbol)
        }
    }
}
