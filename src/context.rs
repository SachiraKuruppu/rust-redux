use std::{any::Any, boxed::Box};
use std::marker::PhantomData;
use rust_redux::Reducer;


pub trait Selector<S> {
    fn select(&self, store: &S) -> Box<dyn Any>;
    fn callback(&mut self, data: Box<dyn Any>);
}

struct SelectorImpl<S,T> {
    selector: Box<dyn Fn(&S) -> T>,
    callback: Box<dyn FnMut(Box<T>)>,
}

impl<S,T> SelectorImpl<S,T>{
    fn new<F,G>(selector: F, callback: G) -> Self
    where
        F: Fn(&S) -> T + 'static,
        G: FnMut(Box<T>) + 'static,
    {
        SelectorImpl {
            selector: Box::new(selector),
            callback: Box::new(callback),
        }
    }
}

impl<S,T: 'static> Selector<S> for SelectorImpl<S,T> {

    fn select(&self, store: &S) -> Box<dyn Any> {
        let current_value = (self.selector)(store);
        Box::new(current_value)
    }

    fn callback(&mut self, data: Box<dyn Any>) {
        if let Ok(downcasted_box) = data.downcast::<T>() {
            (self.callback)(downcasted_box)
        }
    }
}

pub struct Context<S, A, R>
where
    R: Reducer<S, A>,
{
    pub store: Box<S>,
    _reducer: R,
    _phantom: PhantomData<A>, // Indicates that Context logically depends on A

    selectors: Vec<Box<dyn Selector<S>>>,
}

impl<S: 'static, A, R> Context<S, A, R>
where
    R: Reducer<S, A>,
{
    pub fn new(reducer: R, initial_state: S) -> Self {
        Context {
            store: Box::new(initial_state),
            _reducer: reducer,
            _phantom: PhantomData,

            // subscribers: HashMap<u64, Subscriber<S,T>>::new(),
            selectors: Vec::new(),
        }
    }

    pub fn dispatch(&mut self, action: &A) {
        R::reduce(&mut self.store, action);
        self.notify_selectors();
    }
}

impl<S: 'static, A, R> Context<S, A, R>
where
    R: Reducer<S, A>,
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
