use rust_redux::Reducer;


#[derive(Debug)]
pub struct Store {
    pub first_name: String,
    pub last_name: String,
    pub count: i32,
}

pub enum Action {
    Increment,
    SetFirstName(String),
    SetLastName(String),
}

pub struct MyReducer;
impl Reducer<Store, Action> for MyReducer{
    fn reduce(store: &mut Box<Store>, action: &Action) {
        match action {
            Action::Increment => {
                store.count += 1;
            },

            Action::SetFirstName(name) => {
                store.first_name = name.clone();
            },

            Action::SetLastName(name) => {
                store.last_name = name.clone();
            }
        }
    }
}
