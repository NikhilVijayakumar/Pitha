use pitha_core::CoreResult;

/// Trait for test fixtures that set up and tear down test state.
pub trait Fixture {
    /// Set up the fixture.
    fn setup(&self) -> CoreResult<()>;

    /// Tear down the fixture.
    fn teardown(&self);
}

/// A fixture that runs a setup function and a teardown function.
pub struct FnFixture {
    setup_fn: Box<dyn Fn() -> CoreResult<()>>,
    teardown_fn: Box<dyn Fn()>,
}

impl FnFixture {
    pub fn new<F, G>(setup: F, teardown: G) -> Self
    where
        F: Fn() -> CoreResult<()> + 'static,
        G: Fn() + 'static,
    {
        Self {
            setup_fn: Box::new(setup),
            teardown_fn: Box::new(teardown),
        }
    }
}

impl Fixture for FnFixture {
    fn setup(&self) -> CoreResult<()> {
        (self.setup_fn)()
    }

    fn teardown(&self) {
        (self.teardown_fn)()
    }
}

impl<F, G> From<(F, G)> for FnFixture
where
    F: Fn() -> CoreResult<()> + 'static,
    G: Fn() + 'static,
{
    fn from((setup, teardown): (F, G)) -> Self {
        Self::new(setup, teardown)
    }
}
