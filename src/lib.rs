pub trait Reducer<S, A> {
    fn reduce(store: &mut Box<S>, action: &A);
}

// pub trait Subscriber<S, T> {
//     fn select(store: &Box<S>) -> T
//     where
//         T: Hash;
    
//     fn callback(value: T);
// }

// pub trait Subscriber {
//     type Output: Hash + Eq;

//     fn select(store: Box<dyn std::any::Any>) -> Self::Output;
//     fn callback(value: Self::Output);
// }

// pub trait SubscriberBase<S>
// {
//     fn get_previous_hash(&self) -> Option<u64>;
//     fn select(&self, store: &Box<S>) -> dyn Any;
//     fn callback(new_value: dyn Any);
// }

// pub struct Subscriber<T>
// where
//     T: Hash + Eq,
// {
//     previous_value: Option<T>,
// }

// impl<T> Subscriber<T>
// where
//     T: Hash + Eq,
// {
//     pub fn new(select: Box<dyn Fn(&Box<S>) -> T>, callback: Box<dyn Fn(T)>) -> Self {
//         Subscriber {
//             previous_value_hash: None,
//         }
//     }
// }

// impl<S, T> SubscriberBase<S> for Subscriber<T>
// where
//     T: Hash + Eq + 'static,
// {
//     fn get_previous_hash(&self) -> Option<u64> {
//         Some(1234 as u64)
//     }


// }
