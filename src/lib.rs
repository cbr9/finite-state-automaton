use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct FSA<'a, Q> {
    accept_states: &'a [Q],
    transition_matrix: Vec<Vec<Option<Q>>>,
    start_state: Q,
    state_to_index: HashMap<Q, usize, RandomState>,
    symbol_to_index: HashMap<String, usize, RandomState>,
}

impl<'a, Q: Debug + Eq + Hash + Clone> FSA<'a, Q> {
    pub fn new<S: ToString>(
        states: &'a [Q],
        symbols: &'a [S],
        start_state: Q,
        accept_states: &'a [Q],
    ) -> Self {
        assert!(states.contains(&start_state));
        assert!(
            accept_states.iter().all(|state| states.contains(state))
                && accept_states.len() <= states.len()
        );

        let state_to_index = HashMap::from_iter(
            states
                .into_iter()
                .enumerate()
                .map(|(index, state)| (state.clone(), index)),
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
            transition_matrix: vec![vec![None; symbols.len()]; states.len()],
            state_to_index,
            symbol_to_index,
        }
    }

    pub fn transition<S: ToString>(mut self, from: Q, to: Q, on: &[S]) -> Self {
        let from_index = self.state_to_index[&from];
        for symbol in on {
            let symbol_index = self.symbol_to_index[&symbol.to_string()];
            self.transition_matrix[from_index][symbol_index].replace(to.clone());
        }
        self
    }

    pub fn accepts<P: ToString + Hash + Eq, T: Iterator<Item = P>>(&self, tape: T) -> bool {
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

    pub fn rejects<P: ToString + Hash + Eq, T: Iterator<Item = P>>(&self, tape: T) -> bool {
        !self.accepts(tape)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sheeptalk() {
        //           INPUT
        // STATE   b   a   !
        //   0     1   ∅   ∅
        //   1     ∅   2   ∅
        //   2     ∅   3   ∅
        //   3     ∅   3   4
        //   4:    ∅   ∅   ∅
        let fsa = FSA::new(&[0, 1, 2, 3, 4], &['b', 'a', '!'], 0, &[4])
            .transition(0, 1, &['b'])
            .transition(1, 2, &['a'])
            .transition(2, 3, &['a'])
            .transition(3, 3, &['a'])
            .transition(3, 4, &['!']);

        assert!(fsa.accepts("baa!".chars()));
        assert!(fsa.rejects("ba!".chars()));
        assert!(fsa.accepts("baaaaa!".chars()));
        assert!(fsa.rejects(1234345.to_string().chars()));
    }
}