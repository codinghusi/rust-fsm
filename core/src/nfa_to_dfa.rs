use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use crate::dfa::{DFA, DFAState};
use crate::nfa::{NFA, NFAState, StateId};

#[derive(Eq, PartialEq)]
struct MetaState(HashSet<StateId>);

impl Hash for MetaState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.iter().for_each(|state_id| state_id.hash(state)); // FIXME: Bad, O(n) is not useful here :(
    }
}

impl Into<DFA> for NFA {
    fn into(self) -> DFA {
        enum Id {
            New(StateId),
            Existed(StateId)
        }

        impl Id {
            pub fn unpack(self) -> StateId {
                match self {
                    Self::New(id) |
                    Self::Existed(id) => id
                }
            }
        }

        let mut id_mapping: HashMap<MetaState, StateId> = HashMap::new();
        let mut states: HashMap<StateId, DFAState> = HashMap::new();

        let mut get_state_id = |meta_state: HashSet<StateId>| -> Id {
            if let Some(id) = id_mapping.get(&MetaState(meta_state.clone())) {
                Id::Existed(*id)
            } else {
                let new_id = id_mapping.len();
                id_mapping.insert(MetaState(meta_state), new_id);
                Id::New(new_id)
            }
        };

        let mut to_visit = VecDeque::new();
        to_visit.push_back(self.start_states.clone());

        let new_start_state = get_state_id(self.start_states.clone()).unpack();
        let mut new_end_states = HashSet::new();


        while let Some(meta_state) = &to_visit.pop_front() {

            // map meta state to id
            let meta_state_id = get_state_id(meta_state.clone()).unpack();

            let mut new_raw_state: NFAState = HashMap::new();

            // merge all transitions
            for old_state_id in meta_state {
                for (terminal, to_state_ids) in self.table.get(&old_state_id).unwrap() {
                    if let Some(state) = new_raw_state.get_mut(&terminal) {
                        state.extend(to_state_ids);
                    } else {
                        new_raw_state.insert(*terminal, to_state_ids.clone());
                    }
                }
            }

            // visit new meta states
            for (_, to_state_ids) in new_raw_state.iter() {
                if let Id::New(_) = get_state_id(to_state_ids.clone()) {
                    to_visit.push_back(to_state_ids.clone());
                }
            }

            // convert meta states to its ids so we have a dfa state
            let new_dfa_state = new_raw_state
                .into_iter()
                .map(|(terminal, meta_state)| (
                    terminal,
                    get_state_id(meta_state).unpack()
                ))
                .collect();

            // add potential end_states
            for state_id in meta_state {
                if self.is_end_state(&state_id) {
                    new_end_states.insert(meta_state_id);
                    break;
                }
            }


            states.insert(meta_state_id, new_dfa_state);
        }

        DFA {
            table: states,
            start_state: Some(new_start_state),
            end_states: new_end_states
        }
    }
}