pub mod turing_machine;

fn main() {
    use turing_machine::turing_machine;

    let mut ruleset: Vec<turing_machine::Rule> = Vec::new();
    ruleset.push(turing_machine::Rule {
        state: "A".to_string(),
        instructions: vec![
            turing_machine::Instruction {
                symbol: '2',
                action: turing_machine::Action::move_head('L'),
            },
            turing_machine::Instruction {
                symbol: '1',
                action: turing_machine::Action::Write('1'),
            },
            turing_machine::Instruction {
                symbol: ' ',
                action: turing_machine::Action::Write('S'),
            },
        ],
        next_state: "B".to_string(),
    });

    ruleset.push(turing_machine::Rule {
        state: "B".to_string(),
        instructions: vec![
            turing_machine::Instruction {
                symbol: '2',
                action: turing_machine::Action::Write('2'),
            },
            turing_machine::Instruction {
                symbol: '1',
                action: turing_machine::Action::move_head('R'),
            },
            turing_machine::Instruction {
                symbol: 'S',
                action: turing_machine::Action::write('M')
            },
        ],
        next_state: "A".to_string(),
    });


    let tm: turing_machine::TuringMachine = turing_machine::TuringMachine::new(
        ruleset,
        "A".to_string(),
        1,
    );

    println!("{:?}", tm.execute().tape);
}
