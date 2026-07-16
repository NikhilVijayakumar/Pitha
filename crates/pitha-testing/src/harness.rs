use std::time::Duration;

use crate::fixtures::Fixture;

/// Test harness for managing test lifecycle and context.
pub struct TestHarness {
    name: String,
    fixtures: Vec<Box<dyn Fixture>>,
}

impl TestHarness {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            fixtures: Vec::new(),
        }
    }

    /// Register a fixture for this test harness.
    pub fn with_fixture(mut self, fixture: impl Fixture + 'static) -> Self {
        self.fixtures.push(Box::new(fixture));
        self
    }

    /// Get the test name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Setup all fixtures.
    pub fn setup(&self) -> pitha_core::CoreResult<()> {
        for fixture in &self.fixtures {
            fixture.setup()?;
        }
        Ok(())
    }

    /// Teardown all fixtures in reverse order.
    pub fn teardown(&self) {
        for fixture in self.fixtures.iter().rev() {
            fixture.teardown();
        }
    }

    /// Run a test with automatic setup and teardown.
    pub fn run<F, R>(&self, test: F) -> pitha_core::CoreResult<R>
    where
        F: FnOnce() -> pitha_core::CoreResult<R>,
    {
        self.setup()?;
        let result = test();
        self.teardown();
        result
    }
}

/// Assert that an operation completes within a timeout.
pub fn assert_timeout<F>(timeout: Duration, operation: F)
where
    F: FnOnce(),
{
    let start = std::time::Instant::now();
    operation();
    let elapsed = start.elapsed();
    assert!(
        elapsed <= timeout,
        "operation took {elapsed:?}, expected <= {timeout:?}"
    );
}
