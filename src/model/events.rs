use crate::*;

pub struct Events<T: Clone> {
    handlers: Arc<Mutex<Vec<std::sync::Weak<Box<dyn Fn(T) + Sync + Send>>>>>,
}

impl<T: Clone> Default for Events<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Events<T> {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub fn subscribe(&self, handler: std::sync::Weak<Box<dyn Fn(T) + Sync + Send>>) {
        self.handlers.lock().unwrap().push(handler);
    }
    pub fn fire(&mut self, event: T) {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.retain(|handler| match handler.upgrade() {
            Some(handler) => {
                handler(event.clone());
                true
            }
            None => false,
        });
    }
}
