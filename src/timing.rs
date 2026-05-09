// SPDX-License-Identifier: MIT OR Apache-2.0
//! Timing instrumentation primitives for the `--timing` opt-in stderr output.

use serde::Serialize;
use std::time::Instant;

/// Aggregated phase timings emitted under the `timing` envelope key.
#[derive(Default, Serialize)]
pub struct Timings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discover_ms: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_ms: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_ms: Option<u128>,
    pub elapsed_ms: u128,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Single-shot phase timer; consume with `stop()` to obtain elapsed milliseconds.
pub struct PhaseTimer {
    start: Instant,
}

impl PhaseTimer {
    pub fn start() -> Self {
        Self { start: Instant::now() }
    }

    pub fn stop(self) -> u128 {
        self.start.elapsed().as_millis()
    }
}

/// Top-level JSON envelope: `{"timing": {...}}`.
#[derive(Serialize)]
pub struct TimingEnvelope {
    pub timing: Timings,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_full_success() {
        let env = TimingEnvelope {
            timing: Timings {
                discover_ms: Some(10),
                parse_ms: Some(2),
                write_ms: Some(1),
                elapsed_ms: 13,
                error: None,
            },
        };
        let s = serde_json::to_string(&env).unwrap();
        // Single-line property: no embedded newlines, no leading/trailing whitespace.
        assert!(!s.contains('\n'), "JSON should not contain raw newline byte: {s}");
        assert_eq!(s, s.trim(), "JSON should not have leading/trailing whitespace: {s}");
        // Structural value check via serde_json::Value round-trip.
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["timing"]["discover_ms"], 10);
        assert_eq!(v["timing"]["parse_ms"], 2);
        assert_eq!(v["timing"]["write_ms"], 1);
        assert_eq!(v["timing"]["elapsed_ms"], 13);
        assert!(v["timing"].get("error").is_none(), "expected no error field: {s}");
    }

    #[test]
    fn test_serialize_partial_failure() {
        let env = TimingEnvelope {
            timing: Timings {
                discover_ms: Some(10),
                parse_ms: None,
                write_ms: None,
                elapsed_ms: 11,
                error: Some("boom".into()),
            },
        };
        let s = serde_json::to_string(&env).unwrap();
        assert!(s.contains("discover_ms"), "expected discover_ms in {s}");
        assert!(s.contains("elapsed_ms"), "expected elapsed_ms in {s}");
        assert!(s.contains("error"), "expected error in {s}");
        assert!(!s.contains("parse_ms"), "did not expect parse_ms in {s}");
        assert!(!s.contains("write_ms"), "did not expect write_ms in {s}");
    }

    #[test]
    fn test_serialize_escapes_special_chars() {
        let original = "quote\"backslash\\newline\nend";
        let env = TimingEnvelope {
            timing: Timings {
                discover_ms: None,
                parse_ms: None,
                write_ms: None,
                elapsed_ms: 0,
                error: Some(original.to_string()),
            },
        };
        let s = serde_json::to_string(&env).unwrap();
        assert!(!s.contains('\n'), "JSON should not contain raw newline byte: {s}");
        let v: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert_eq!(v["timing"]["error"].as_str().unwrap(), original);
    }
}
