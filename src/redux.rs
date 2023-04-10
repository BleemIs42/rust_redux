// Define the Redux store
pub struct Store<S, A> {
    state: S,
    reducer: fn(&mut S, A),
    subscribers: Vec<Box<dyn Fn()>>,
    middleware: Vec<Box<dyn Fn(&mut Store<S, A>, A, &dyn Fn(A))>>,
}

impl<S, A> Store<S, A> {
    pub fn new(reducer: fn(&mut S, A), initial_state: S) -> Store<S, A> {
        Store {
            state: initial_state,
            reducer,
            subscribers: Vec::new(),
            middleware: Vec::new(),
        }
    }

    pub fn dispatch(&mut self, action: A) {
        let mut dispatch_fn = |a: A| {
            (self.reducer)(&mut self.state, a);
            for subscriber in &self.subscribers {
                subscriber();
            }
        };
        for middleware in &self.middleware {
            let old_dispatch_fn = dispatch_fn;
            dispatch_fn = move |a: A| middleware(self, a, &old_dispatch_fn);
        }
        dispatch_fn(action);
    }

    pub fn get_state(&self) -> &S {
        &self.state
    }

    pub fn subscribe(&mut self, subscriber: Box<dyn Fn()>) {
        self.subscribers.push(subscriber);
    }

    pub fn apply_middleware(&mut self, middleware: Box<dyn Fn(&mut Store<S, A>, A, &dyn Fn(A))>) {
        self.middleware.push(middleware);
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

    #[test]
    fn test_subscribe() {
        let initial_state = State { counter: 0 };
        let mut store = Store::new(reducer, initial_state);
        let mut count = 0;
        let subscriber = || count += 1;
        store.subscribe(Box::new(subscriber));
        store.dispatch(Action::Increment);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_middleware() {
        let initial_state = State { counter: 0 };
        let mut store = Store::new(reducer, initial_state);
        let middleware = |store: &mut Store<State, Action>, action: Action, next: &dyn Fn(Action)| {
            store.dispatch(Action::Increment);
            next(action);
        };
        store.apply_middleware(Box::new(middleware));
        store.dispatch(Action::Decrement);
        let current_state = store.get_state();
        assert_eq!(current_state.counter, 1);
    }
}

