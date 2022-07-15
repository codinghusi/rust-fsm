use std::collections::{HashMap, HashSet};

type StateId = usize;
type Terminal = char;
pub type DFAState = HashMap<Terminal, StateId>;

#[derive(Debug, Eq, PartialEq)]
pub struct DFA {
    pub table: HashMap<StateId, DFAState>,
    pub start_state: Option<StateId>,
    pub end_states: HashSet<StateId>
}

impl DFA {
    pub fn new() -> DFA {
        DFA {
            table: HashMap::new(),
            start_state: None,
            end_states: HashSet::new()
        }
    }

    pub fn add_transition(&mut self, from_id: &StateId, to_id: StateId, terminal: Terminal) -> Result<(), String> {
        if !self.is_state(to_id) {
            Err(format!("'to' state {} doesn't exist in this DFA.", to_id))?
        }

        let state = self.table
            .get_mut(from_id)
            .ok_or(format!("'from' state {} doesn't exist in this DFA.", from_id))?;

        if state.contains_key(&terminal) {
            Err(format!("there is already a transition from {} to {} with character {}, if you wanted to override it to state {}, use set_transition", from_id, terminal, terminal, to_id))?
        }

        state.insert(terminal, to_id);

        Ok(())
    }

    pub fn set_transition(&mut self, from: &StateId, to: StateId, terminal: Terminal) -> Result<(), String> {
        if !self.is_state(to) {
            Err(format!("'to' state {} doesn't exist in this DFA.", to))?
        }

        let state = self.table
            .get_mut(from)
            .ok_or(format!("'from' state {} doesn't exist in this DFA.", from))?;

        state.insert(terminal, to);

        Ok(())
    }

    pub fn add_state(&mut self) -> StateId {
        let transition = HashMap::new();
        let state_id = self.table.len();
        self.table.insert(state_id, transition);
        state_id
    }

    pub fn is_state(&self, state: StateId) -> bool {
        state < self.table.len()
    }

    pub fn set_start_state(&mut self, state: StateId) -> Result<(), String> {
        if !self.is_state(state) {
            Err(format!("state {} requested to be start state is not a state of DFA.", state))
        } else {
            self.start_state = Some(state);
            Ok(())
        }
    }

    pub fn set_end_states(&mut self, states: HashSet<StateId>) -> Result<(), String> {
        if let Some(state) = states.iter().find(|state| !self.is_state(**state)) {
            Err(format!("state {} requested to be end state is not a state of DFA.", state))
        } else {
            self.end_states = states;
            Ok(())
        }
    }
}


