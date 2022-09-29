pub struct Memoized<F, T>(State<F, T>) where F: FnOnce() -> T;

impl<F, T> Memoized<F, T> where F: FnOnce() -> T {
    pub fn new(f: F) -> Self {
        Self(State::Uninit(Some(f)))
    }

    pub fn get(&mut self) -> &T {
        if let State::Uninit(f) = &mut self.0 {
            let val = f.take().unwrap()();
            self.0 = State::Init(val);
        }

        self.try_get().expect("Cannot be uninitialised")
    }

    pub fn try_get(&self) -> Option<&T> {
        match &self.0 {
            State::Uninit(_) => None,
            State::Init(val) => Some(val)
        }
    }
}

enum State<F, T> where F: FnOnce() -> T {
    Uninit(Option<F>),
    Init(T)
}

#[cfg(test)]
mod tests;