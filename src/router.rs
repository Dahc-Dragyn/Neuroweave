use regex::Regex;
use std::sync::OnceLock;
use crate::ipc::IntentPayload;
use crate::confidence::{ConfidenceProfile, AmbiguityFlag};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskID {
    FormatLint,
    ScaffoldModule,
    CleanImports,
}

pub fn evaluate_intent(payload: &IntentPayload) -> ConfidenceProfile {
    static RE_SEMANTIC_DRIFT: OnceLock<Regex> = OnceLock::new();
    static RE_FORMAT_LINT: OnceLock<Regex> = OnceLock::new();
    static RE_SCAFFOLD_INIT: OnceLock<Regex> = OnceLock::new();
    static RE_CLEAN: OnceLock<Regex> = OnceLock::new();
    static RE_IMPORTS: OnceLock<Regex> = OnceLock::new();

    // Step 1: Strict Layer 4 Check (First priority) - Expanded subjective vocabulary list
    let drift = RE_SEMANTIC_DRIFT.get_or_init(|| {
        Regex::new(r"(?i)improve|refactor|simplify|modernize|clean\s+this\s+up|optimize|architect|design|rework|bulletproof|harden|audit|review|evaluate|fix|debug|resolve|advise|integrate").unwrap()
    });

    let prompt = &payload.raw_prompt;

    if drift.is_match(prompt) {
        return ConfidenceProfile {
            task_id: None,
            confidence_score: 0.0,
            ambiguity_flags: vec![AmbiguityFlag::SubjectiveLanguage],
            escalation_reason: Some("Subjective language detected. Escalating for semantic interpretation.".to_string()),
        };
    }

    // Step 2: Layer 1 Lexical Check (Only if Step 1 passes)
    let format_lint = RE_FORMAT_LINT.get_or_init(|| Regex::new(r"(?i)format|lint").unwrap());
    let scaffold_init = RE_SCAFFOLD_INIT.get_or_init(|| Regex::new(r"(?i)scaffold|init").unwrap());
    let clean = RE_CLEAN.get_or_init(|| Regex::new(r"(?i)clean").unwrap());
    let imports = RE_IMPORTS.get_or_init(|| Regex::new(r"(?i)imports").unwrap());

    if format_lint.is_match(prompt) {
        ConfidenceProfile {
            task_id: Some(TaskID::FormatLint),
            confidence_score: 0.95,
            ambiguity_flags: vec![],
            escalation_reason: None,
        }
    } else if scaffold_init.is_match(prompt) {
        ConfidenceProfile {
            task_id: Some(TaskID::ScaffoldModule),
            confidence_score: 0.95,
            ambiguity_flags: vec![],
            escalation_reason: None,
        }
    } else if clean.is_match(prompt) && imports.is_match(prompt) {
        ConfidenceProfile {
            task_id: Some(TaskID::CleanImports),
            confidence_score: 0.95,
            ambiguity_flags: vec![],
            escalation_reason: None,
        }
    } else {
        // Step 3: Default Fallback (Cache Miss)
        ConfidenceProfile {
            task_id: None,
            confidence_score: 0.0,
            ambiguity_flags: vec![],
            escalation_reason: Some("No matching subroutines found in local registry.".to_string()),
        }
    }
}
