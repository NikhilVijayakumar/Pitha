use std::fmt;
use std::time::Duration;

/// Engineering evidence produced by test execution.
#[derive(Debug, Clone)]
pub struct Evidence {
    pub test_name: String,
    pub result: TestResult,
    pub duration: Duration,
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestResult {
    Passed,
    Failed,
    Skipped,
    Error,
}

impl fmt::Display for TestResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Passed => write!(f, "PASSED"),
            Self::Failed => write!(f, "FAILED"),
            Self::Skipped => write!(f, "SKIPPED"),
            Self::Error => write!(f, "ERROR"),
        }
    }
}

impl Evidence {
    pub fn passed(test_name: impl Into<String>, duration: Duration) -> Self {
        Self {
            test_name: test_name.into(),
            result: TestResult::Passed,
            duration,
            message: None,
        }
    }

    pub fn failed(
        test_name: impl Into<String>,
        duration: Duration,
        message: impl Into<String>,
    ) -> Self {
        Self {
            test_name: test_name.into(),
            result: TestResult::Failed,
            duration,
            message: Some(message.into()),
        }
    }
}

/// Formatter for evidence output.
pub struct EvidenceFormatter;

impl EvidenceFormatter {
    pub fn format_text(evidence: &[Evidence]) -> String {
        let mut output = String::new();
        let total = evidence.len();
        let passed = evidence
            .iter()
            .filter(|e| e.result == TestResult::Passed)
            .count();
        let failed = evidence
            .iter()
            .filter(|e| e.result == TestResult::Failed)
            .count();

        output.push_str(&format!(
            "Test Results: {passed}/{total} passed, {failed} failed\n\n"
        ));

        for e in evidence {
            output.push_str(&format!(
                "[{}] {} ({:?})\n",
                e.result, e.test_name, e.duration
            ));
            if let Some(msg) = &e.message {
                output.push_str(&format!("  {msg}\n"));
            }
        }

        output
    }

    pub fn format_json(evidence: &[Evidence]) -> String {
        let results: Vec<serde_json::Value> = evidence
            .iter()
            .map(|e| {
                serde_json::json!({
                    "test_name": e.test_name,
                    "result": e.result.to_string(),
                    "duration_ms": e.duration.as_millis(),
                    "message": e.message,
                })
            })
            .collect();

        serde_json::to_string_pretty(&results).unwrap_or_default()
    }
}
