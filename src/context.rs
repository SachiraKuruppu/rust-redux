use std::{any::Any, boxed::Box};
use std::marker::PhantomData;
use rust_redux::{Reducer, Selector};

use crate::selector::SelectorImpl;


pub struct Context<S,A,R>
where
    R: Reducer<S,A>,
{
    pub store: Box<S>,
    _reducer: R,
    _phantom: PhantomData<A>, // Indicates that Context logically depends on A

    selectors: Vec<Box<dyn Selector<S>>>,
}

impl<S: 'static,A,R> Context<S,A,R>
where
    R: Reducer<S,A>,
{
    pub fn new(reducer: R, initial_state: S) -> Self {
        Context {
            store: Box::new(initial_state),
            _reducer: reducer,
            _phantom: PhantomData,

            selectors: Vec::new(),
        }
    }

    pub fn dispatch(&mut self, action: &A) {
        R::reduce(&mut self.store, action);
        self.notify_selectors();
    }
}

impl<S: 'static,A,R> Context<S,A,R>
where
    R: Reducer<S,A>,
{
    pub fn use_selector<F,G,T>(&mut self, selector: F, callback: G)
    where
        F: Fn(&S) -> T + 'static,
        G: FnMut(Box<T>) + 'static,
        T: Any + 'static,
    {
        let selector = SelectorImpl::new(
            Box::new(selector),
            Box::new(callback),
        );

        self.selectors.push(Box::new(selector));
    }

    fn notify_selectors(&mut self) {
        for subscriber in self.selectors.iter_mut() {
            let current_value = subscriber.select(&self.store);
            subscriber.callback(current_value);
        }
    }
}
