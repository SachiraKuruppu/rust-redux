use std::any::Any;

pub trait Reducer<S, A> {
    fn reduce(store: &mut Box<S>, action: &A);
}

pub trait Selector<S> {
    fn select(&self, store: &S) -> Box<dyn Any>;
    fn callback(&mut self, data: Box<dyn Any>);
}

mod context;
mod selector;

pub use context::*;
