mod context;
mod store;
mod selector;

use context::Context;
use store::{Action, MyReducer, Store};


fn main() {
    let mut cx = Context::new(
        MyReducer, 
        Store {
            first_name: String::from("Sachira"),
            last_name: String::from("Kuruppu"),
            count: 0,
        }
    );

    (|| {
        let mut v = 0;
        cx.use_selector(
            |store| {
                store.count
            },
            Box::new(move |value| {
                v += 10;
                println!("{}", v);
                println!("New value {:?}",  value)
            }),
        );
    })();

    cx.dispatch(&Action::Increment);
    cx.dispatch(&Action::SetFirstName(String::from("Bob")));
    cx.dispatch(&Action::SetLastName(String::from("Smith")));
    cx.dispatch(&Action::Increment);

    println!("{:#?}", cx.store);
}
