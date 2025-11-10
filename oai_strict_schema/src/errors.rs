use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingKind {
    Error,
    Warning,
}

#[derive(Debug, Clone)]
pub struct Finding {
    /// JSON Pointer into the schema (e.g. "/properties/user/properties/id")
    pub pointer: String,
    pub kind: FindingKind,
    pub message: String,
}

#[derive(Debug, Clone, Default)]
pub struct Report {
    pub findings: Vec<Finding>,
}

impl Report {
    pub fn is_ok(&self) -> bool {
        self.findings.iter().all(|f| matches!(f.kind, FindingKind::Warning))
    }
    pub fn has_errors(&self) -> bool {
        self.findings.iter().any(|f| matches!(f.kind, FindingKind::Error))
    }
    pub fn push(&mut self, pointer: String, kind: FindingKind, message: impl Into<String>) {
        self.findings.push(Finding { pointer, kind, message: message.into() });
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.findings.is_empty() {
            return write!(f, "No issues found.");
        }
        for (i, finding) in self.findings.iter().enumerate() {
            writeln!(f, "{:>3}. [{}] {} — {}", i + 1,
                     match finding.kind { FindingKind::Error => "ERROR", FindingKind::Warning => "WARN" },
                     finding.pointer, finding.message)?;
        }
        Ok(())
    }
}
