use std::collections::HashSet;

trait FiniteAutomata {
    type StateId = usize;
    type Symbol = char;

    fn add_state(&mut self) -> Self::StateId;
    fn is_start_state(&self, state_id: &Self::StateId);
    fn is_end_state(&self, state_id: &Self::StateId);
    fn is_state(&self, state_id: &Self::StateId);
}

trait DeterministicFiniteAutomata: FiniteAutomata {
    fn get_start_state(&self) -> Self::StateId;
    fn get_end_states(&self) -> HashSet<Self::StateId>;
    fn get_next_state(&self, from_state_id: &Self::StateId, symbol: &Self::Symbol)
        -> Self::StateId;
}

trait NonDeterministicFiniteAutomata: FiniteAutomata {
    fn get_start_states(&self) -> HashSet<Self::StateId>;
    fn get_end_states(&self) -> HashSet<Self::StateId>;
    fn get_next_states(
        &self,
        from_state_id: &Self::StateId,
        symbol: &Self::Symbol,
    ) -> HashSet<Self::StateId>;
}

struct DFAInstance<'a, T, I>
where
    T: DeterministicFiniteAutomata,
    I: Iterator<Item = <T as FiniteAutomata>::Symbol>,
{
    pos: usize,
    current_state_id: <T as FiniteAutomata>::StateId,
    dfa: &'a T,
    input: &'a I,
    eof: bool,
}

impl<'a, T> DFAInstance<'a, T>
where
    T: DeterministicFiniteAutomata,
{
    fn create(dfa: &'a T, input: &'a str) -> Self {
        Self {
            pos: 0,
            eof: false,
            current_state_id: dfa.get_start_state(),
            dfa,
            input,
        }
    }

    fn next(&mut self) {
        if let Some(symbol) = input.next() {
            self.current_state_id = self.dfa.get_next_state(self.current_state_id, symbol);
            pos += 1;
        } else {
            self.eof = true;
        }
    }

    fn isEof(&self) -> bool {
        self.eof
    }
}
