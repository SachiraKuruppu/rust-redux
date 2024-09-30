use std::any::Any;
use crate::Selector;

pub struct SelectorImpl<S,T> {
    selector: Box<dyn Fn(&S) -> T>,
    callback: Box<dyn FnMut(Box<T>)>,
}

impl<S,T> SelectorImpl<S,T> {
    pub fn new<F,G>(selector: F, callback: G) -> Self
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
