use crate::dfa::DFA;
use crate::nfa::NFA;

mod dfa;
mod nfa;
mod nfa_to_dfa;

fn main() -> Result<(), String> {

    let mut nfa = NFA::new();

    let a = nfa.add_state();
    let b = nfa.add_state();
    let c = nfa.add_state();

    nfa.add_transition(&a, &a, '0')?;
    nfa.add_transition(&a, &a, '1')?;
    nfa.add_transition(&a, &b, '0')?;
    nfa.add_transition(&b, &c, '1')?;

    let start_states = [a];
    nfa.set_start_states(start_states.iter().copied().collect())?;

    let end_states = [c];
    nfa.set_end_states(end_states.iter().copied().collect())?;

    println!("{:#?}\n", nfa);

    let dfa: DFA = nfa.into();
    println!("\n{:#?}", dfa);

    Ok(())
}
