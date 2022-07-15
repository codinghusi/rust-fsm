use std::collections::{HashSet, HashMap, VecDeque};
use std::hash::{Hash, Hasher};
use crate::dfa::DFA;

pub type StateId = usize;
pub type Terminal = char;

pub type NFAState = HashMap<Terminal, HashSet<StateId>>;

#[derive(Debug)]
pub struct NFA {
    pub table: HashMap<StateId, NFAState>,
    pub start_states: HashSet<StateId>,
    pub end_states: HashSet<StateId>
}

impl NFA {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            start_states: HashSet::new(),
            end_states: HashSet::new()
        }
    }

    pub fn add_transition(&mut self, from_id: &StateId, to_id: &StateId, terminal: Terminal) -> Result<(), String> {
        if !self.is_state(to_id) {
            Err(format!("'to' state {} doesn't exist in this NFA.", to_id))?
        }

        let state = self.table
            .get_mut(from_id)
            .ok_or(format!("'from' state {} doesn't exist in this NFA.", from_id))?;

        let transition_set = if let Some(set) = state.get_mut(&terminal) {
            set
        } else {
            let mut set = HashSet::new();
            state.insert(terminal, set);
            state.get_mut(&terminal).unwrap()
        };

        transition_set.insert(*to_id); // TODO: maybe raise error if transition did already exist

        Ok(())
    }

    pub fn add_state(&mut self) -> StateId {
        let state = HashMap::new();
        let state_id = self.table.len();
        self.table.insert(state_id, state);
        state_id
    }

    pub fn get_state(&self, state_id: &StateId) -> Result<&NFAState, String> {
        if let Some(state) = self.table.get(state_id) {
            Ok(state)
        } else {
            Err(format!("State with id {} doesn't exist in this NFA", state_id))
        }
    }

    pub fn is_state(&self, state_id: &StateId) -> bool {
        self.table.contains_key(state_id)
    }

    pub fn is_end_state(&self, state_id: &StateId) -> bool {
        self.end_states.contains(state_id)
    }

    pub fn set_start_states(&mut self, states: HashSet<StateId>) -> Result<(), String> {
        if let Some(state) = states.iter().find(|state| !self.is_state(*state)) {
            Err(format!("state {} requested to be start state is not a state of NFA", state))
        } else {
            self.start_states = states;
            Ok(())
        }
    }

    pub fn set_end_states(&mut self, states: HashSet<StateId>) -> Result<(), String> {
        if let Some(state) = states.iter().find(|state| !self.is_state(*state)) {
            Err(format!("state {} requested to be end state is not a state of NFA", state))
        } else {
            self.end_states = states;
            Ok(())
        }
    }

    pub fn get_transitions(&self) -> HashSet<(StateId, StateId, Terminal)> {
        self.table
            .iter()
            .flat_map(|(from_id, from)|
                from.iter().map(
                    move |to_id| (from_id, to_id)
                )
            )
            .flat_map(|(from_id, (terminal, transitions))|
                transitions.iter().map(
                    move |to_id| (from_id, to_id, terminal)
                )
            )
            .map(|(from_id, to_id, terminal)| (*from_id, *to_id, *terminal))
            .collect()
    }
}

