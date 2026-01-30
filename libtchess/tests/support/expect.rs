use std::fmt::Debug;

#[allow(unused)]
pub struct Expect<T, TT> {
    setup: Box<dyn Fn() -> TT>,
    subject_fn: Option<Box<dyn Fn(&mut TT)>>,
    change_fn: Option<Box<dyn Fn(&mut TT) -> T>>,
}

impl<T: PartialEq + Debug, TT> Expect<T, TT> {
    #[allow(unused)]
    pub fn setup<F>(setup_fn: F) -> Self
    where
        F: Fn() -> TT + 'static,
    {
        Self {
            setup: Box::new(setup_fn),
            subject_fn: None,
            change_fn: None,
        }
    }

    pub fn setup_fn(&self) -> &Box<dyn Fn() -> TT> {
        &self.setup
    }

    pub fn subject_fn(&self) -> Option<&Box<dyn Fn(&mut TT)>> {
        self.subject_fn.as_ref()
    }

    pub fn change_fn(&self) -> Option<&Box<dyn Fn(&mut TT) -> T>> {
        self.change_fn.as_ref()
    }

    #[allow(unused)]
    pub fn expect<F>(&mut self, subject_fn: F) -> &mut Self
    where
        F: Fn(&mut TT) + 'static,
    {
        self.subject_fn = Some(Box::new(subject_fn));
        self
    }

    //noinspection RsSelfConvention
    #[allow(unused)]
    pub fn to_change<F>(&mut self, change_fn: F) -> &mut Self
    where
        F: Fn(&mut TT) -> T + 'static,
    {
        self.change_fn = Some(Box::new(change_fn));
        self
    }

    // Executes subject_fn with the given setup and returns the setup object
    #[allow(unused)]
    pub fn run_expectation(&mut self) -> TT {
        let mut setup = self.setup.as_ref()();
        self.subject_fn.as_ref().unwrap()(&mut setup);
        setup
    }
}
