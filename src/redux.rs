// Define the Redux store
pub struct Store<S, A> {
    state: S,
    reducer: fn(&mut S, A),
}

impl<S, A> Store<S, A> {
    pub fn new(reducer: fn(&mut S, A), initial_state: S) -> Store<S, A> {
        Store {
            state: initial_state,
            reducer,
        }
    }

    pub fn dispatch(&mut self, action: A) {
        (self.reducer)(&mut self.state, action);
    }

    pub fn get_state(&self) -> &S {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct State {
        pub counter: i32,
    }
    enum Action {
        Increment,
        Decrement,
    }
    fn reducer(state: &mut State, action: Action) {
        match action {
            Action::Increment => state.counter += 1,
            Action::Decrement => state.counter -= 1,
        }
    }
    #[test]
    fn test_store() {
        let initial_state = State { counter: 0 };
        let mut store = Store::new(reducer, initial_state);
        store.dispatch(Action::Increment);
        store.dispatch(Action::Increment);
        store.dispatch(Action::Decrement);
        let current_state = store.get_state();
        assert_eq!(current_state.counter, 1);
    }
}

