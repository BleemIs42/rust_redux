use redux::Store;

mod redux;

#[derive(Debug)]
pub struct State {
    pub counter: i32,
}

pub enum Action {
    Increment,
    Decrement,
}

pub fn reducer(state: &mut State, action: Action) {
    match action {
        Action::Increment => state.counter += 1,
        Action::Decrement => state.counter -= 1,
    }
}

fn main() {
    let initial_state = State { counter: 0 };
    let mut store = Store::new(reducer, initial_state);
    store.dispatch(Action::Increment);
    let current_state = store.get_state();
    println!("current_state: {:?}", current_state);

    store.dispatch(Action::Increment);
    let current_state = store.get_state();
    println!("current_state: {:?}", current_state);
    
    store.dispatch(Action::Decrement);
    let current_state = store.get_state();
    println!("current_state: {:?}", current_state);
}
