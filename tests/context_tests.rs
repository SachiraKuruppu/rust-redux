use rust_redux::Reducer;
use rust_redux::Context;

#[derive(Debug)]
pub struct Store {
    pub count: i32,
}

pub enum Action {
    Increment,
}

pub struct MyReducer;
impl Reducer<Store, Action> for MyReducer{
    fn reduce(store: &mut Box<Store>, action: &Action) {
        match action {
            Action::Increment => {
                store.count += 1;
            },
        }
    }
}

#[test]
fn test_context_selector_callback() {
    let mut cx = Context::new(
        MyReducer, 
        Store {
            count: 10,
        }
    );

    cx.use_selector(
        |store| {
            store.count
        },
        Box::new(|value: Box<i32>| {
            let count: i32 = *value;
            assert_eq!(count, 10);
        }),
    );
}

#[test]
fn test_reducer_action() {
    let mut cx = Context::new(
        MyReducer, 
        Store {
            count: 10,
        }
    );

    cx.dispatch(&Action::Increment);

    cx.use_selector(
        |store| {
            store.count
        },
        Box::new(|value: Box<i32>| {
            let count: i32 = *value;
            assert_eq!(count, 11);
        }),
    );
}
