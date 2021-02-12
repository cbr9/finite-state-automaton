use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::FromIterator;

struct FSA<Q> {
    transition_matrix: Vec<Vec<Option<Q>>>,
    start_state: Q,
    accept_states: Vec<Q>,
    state_to_index: HashMap<Q, usize, RandomState>,
    symbol_to_index: HashMap<String, usize, RandomState>,
}

impl<Q: Debug + Eq + Hash + Clone> FSA<Q> {
    fn new<S: ToString>(
        states: Vec<Q>,
        symbols: Vec<S>,
        start_state: Q,
        accept_states: Vec<Q>,
        transition_matrix: Vec<Vec<Option<Q>>>,
    ) -> Self {
        assert!(states.contains(&start_state));
        assert!(accept_states.iter().all(|state| states.contains(state)));

        let state_to_index = HashMap::from_iter(
            states
                .into_iter()
                .enumerate()
                .map(|(index, state)| (state, index)),
        );
        let symbol_to_index = HashMap::from_iter(
            symbols
                .into_iter()
                .enumerate()
                .map(|(index, symbol)| (symbol.to_string(), index)),
        );
        Self {
            start_state,
            accept_states,
            transition_matrix,
            state_to_index,
            symbol_to_index,
        }
    }

    fn accepts<P: ToString + Hash + Eq, T: Iterator<Item = P>>(&self, tape: T) -> bool {
        let index_to_item: HashMap<usize, T::Item, RandomState> =
            HashMap::from_iter(tape.into_iter().enumerate());

        let length = index_to_item.len();
        let mut current_input_index = 0;
        let mut current_state = self.start_state.clone();

        loop {
            if current_input_index == length {
                return self.accept_states.contains(&current_state);
            } else {
                let current_state_index = self.state_to_index[&current_state];
                let current_symbol = index_to_item[&current_input_index].to_string();
                match self.symbol_to_index.get(&current_symbol) {
                    Some(current_symbol_index) => {
                        match &self.transition_matrix[current_state_index][*current_symbol_index] {
                            None => return false,
                            Some(next_state) => {
                                current_state = next_state.clone();
                                current_input_index += 1;
                            }
                        }
                    }
                    None => return false, // if the character isn't in the language's alphabet
                }
            }
        }
    }
}

fn main() {
    let fsa = FSA::new(
        vec![0, 1, 2, 3, 4],
        vec!['b', 'a', '!'],
        0,
        vec![4],
        vec![
            //     b        a     !
            vec![Some(1), None, None],
            vec![None, Some(2), None],
            vec![None, Some(3), None],
            vec![None, Some(3), Some(4)],
            vec![None, None, None],
        ],
    );
    assert!(fsa.accepts("baa!".chars()));
    assert!(!fsa.accepts("ba!".chars()));
    assert!(fsa.accepts("baaaaa!".chars()));
    assert!(!fsa.accepts(1234345.to_string().chars()));
}
