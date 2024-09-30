use rust_redux::Reducer;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Increment,
    Default,
}

#[derive(Debug)]
pub struct Store {
    pub count: i32,
    pub action: Action,
}


pub struct MyReducer;
impl Reducer<Store, Action> for MyReducer{
    fn reduce(store: &mut Box<Store>, action: &Action) {
        store.action = action.clone();
    }
}

#[test]
fn test_reducer_reduce() {
    let store = Store {
        count: 10,
        action: Action::Default,
    };

    let mut store_box = Box::new(store);

    MyReducer::reduce(&mut store_box, &Action::Increment);

    assert_eq!(store_box.action, Action::Increment);
}
