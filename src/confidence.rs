#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmbiguityFlag {
    SubjectiveLanguage,
    #[allow(dead_code)]
    UnknownBlastRadius,
    #[allow(dead_code)]
    ArchitecturalModification,
    #[allow(dead_code)]
    IncompleteContext,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConfidenceProfile {
    pub task_id: Option<crate::router::TaskID>,
    pub confidence_score: f32,
    pub ambiguity_flags: Vec<AmbiguityFlag>,
    pub escalation_reason: Option<String>,
}
