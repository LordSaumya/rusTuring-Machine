pub mod turing_machine;

fn main() {
    use turing_machine::turing_machine;

    let alternate_ruleset = vec![turing_machine::Rule {
        state: "Start".to_string(),
        instructions: vec![
            turing_machine::Instruction {
                symbol: ' ',
                action: turing_machine::Action::Write('1'),
            },
            turing_machine::Instruction {
                symbol: '1',
                action: turing_machine::Action::Move(turing_machine::MoveHead::Right),
            },
        ],
        next_state: "Print0".to_string(),
    },
    
    turing_machine::Rule {
        state: "Print0".to_string(),
        instructions: vec![
            turing_machine::Instruction {
                symbol: ' ',
                action: turing_machine::Action::Write('0'),
            },
            turing_machine::Instruction {
                symbol: '0',
                action: turing_machine::Action::Move(turing_machine::MoveHead::Right),
            },
        ],
        next_state: "Print1".to_string(),
    },
    
    turing_machine::Rule {
        state: "Print1".to_string(),
        instructions: vec![
            turing_machine::Instruction {
                symbol: ' ',
                action: turing_machine::Action::Write('1'),
            },
            turing_machine::Instruction {
                symbol: '1',
                action: turing_machine::Action::Move(turing_machine::MoveHead::Right),
            },
        ],
        next_state: "Print0".to_string(),
    }];

    let tm: turing_machine::TuringMachine = turing_machine::TuringMachine::new(
        alternate_ruleset,
        "Start".to_string(),
        1,
    );

    println!("{:?}", tm.execute().tape);
}