//! Pure form validation — parity with `validateField` in `src/ui/mod.rs` (no DOM).

/// Result of validating a single field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldValidation {
    pub valid: bool,
    pub error: Option<String>,
}

impl FieldValidation {
    pub fn ok() -> Self {
        Self {
            valid: true,
            error: None,
        }
    }

    pub fn err(message: impl Into<String>) -> Self {
        Self {
            valid: false,
            error: Some(message.into()),
        }
    }
}

/// Default English messages (JS `T('form.*', fallback)`).
pub mod messages {
    pub const REQUIRED: &str = "This field is required";
    pub const VALID_NUMBER: &str = "Please enter a valid number";
    pub fn value_min(min: &str) -> String {
        format!("Value must be at least {min}")
    }
    pub fn value_max(max: &str) -> String {
        format!("Value must be at most {max}")
    }
    pub const VALID_EMAIL: &str = "Please enter a valid email address";
    pub const INVALID_FORMAT: &str = "Invalid format";
}

/// Required non-empty after trim.
pub fn validate_required(value: &str) -> FieldValidation {
    if value.trim().is_empty() {
        FieldValidation::err(messages::REQUIRED)
    } else {
        FieldValidation::ok()
    }
}

/// Number input with optional min/max (mirrors `input.type === 'number'` branch).
pub fn validate_number(value: &str, min: Option<&str>, max: Option<&str>) -> FieldValidation {
    if value.trim().is_empty() {
        return FieldValidation::ok();
    }
    let Ok(n) = value.trim().parse::<f64>() else {
        return FieldValidation::err(messages::VALID_NUMBER);
    };
    if !n.is_finite() {
        return FieldValidation::err(messages::VALID_NUMBER);
    }
    if let Some(min_s) = min {
        if let Ok(min_v) = min_s.parse::<f64>() {
            if n < min_v {
                return FieldValidation::err(messages::value_min(min_s));
            }
        }
    }
    if let Some(max_s) = max {
        if let Ok(max_v) = max_s.parse::<f64>() {
            if n > max_v {
                return FieldValidation::err(messages::value_max(max_s));
            }
        }
    }
    FieldValidation::ok()
}

/// Email regex parity: `/^[^\s@]+@[^\s@]+\.[^\s@]+$/`
pub fn validate_email(value: &str) -> FieldValidation {
    if value.trim().is_empty() {
        return FieldValidation::ok();
    }
    let v = value.trim();
    let Some((local, domain)) = v.split_once('@') else {
        return FieldValidation::err(messages::VALID_EMAIL);
    };
    if local.is_empty() || domain.is_empty() {
        return FieldValidation::err(messages::VALID_EMAIL);
    }
    let Some((host, tld)) = domain.rsplit_once('.') else {
        return FieldValidation::err(messages::VALID_EMAIL);
    };
    if host.is_empty() || tld.is_empty() || host.contains('@') || local.contains('@') {
        return FieldValidation::err(messages::VALID_EMAIL);
    }
    if local.contains(' ') || domain.contains(' ') {
        return FieldValidation::err(messages::VALID_EMAIL);
    }
    FieldValidation::ok()
}

/// Pattern match via simple substring/char rules; for full regex use `validate_pattern_regex`.
pub fn validate_pattern(
    value: &str,
    pattern: &str,
    pattern_error: Option<&str>,
) -> FieldValidation {
    if value.trim().is_empty() {
        return FieldValidation::ok();
    }
    if pattern_matches(value, pattern) {
        FieldValidation::ok()
    } else {
        FieldValidation::err(pattern_error.unwrap_or(messages::INVALID_FORMAT))
    }
}

/// Minimal pattern support: exact match or `*` wildcard segments (admin uses full RegExp in browser).
fn pattern_matches(value: &str, pattern: &str) -> bool {
    if pattern.is_empty() {
        return true;
    }
    if !pattern.contains('*') {
        return regex_like(value, pattern);
    }
    let parts: Vec<&str> = pattern.split('*').collect();
    let mut rest = value;
    for (i, part) in parts.iter().enumerate() {
        if part.is_empty() {
            continue;
        }
        if i == 0 {
            if !rest.starts_with(part) {
                return false;
            }
            rest = &rest[part.len()..];
        } else if i == parts.len() - 1 {
            if !rest.ends_with(part) {
                return false;
            }
        } else if let Some(pos) = rest.find(part) {
            rest = &rest[pos + part.len()..];
        } else {
            return false;
        }
    }
    true
}

fn regex_like(value: &str, pattern: &str) -> bool {
    if pattern.starts_with('^') && pattern.ends_with('$') && pattern.len() >= 2 {
        let inner = &pattern[1..pattern.len() - 1];
        return value == inner;
    }
    value.contains(pattern)
}

/// Combined field validation (required + type-specific rules).
pub fn validate_field(input: FieldInput<'_>) -> FieldValidation {
    if input.required {
        let r = validate_required(input.value);
        if !r.valid {
            return r;
        }
    }
    if input.kind == FieldKind::Number {
        let n = validate_number(input.value, input.min, input.max);
        if !n.valid {
            return n;
        }
    }
    if input.kind == FieldKind::Email {
        let e = validate_email(input.value);
        if !e.valid {
            return e;
        }
    }
    if let Some(pat) = input.pattern {
        let p = validate_pattern(input.value, pat, input.pattern_error);
        if !p.valid {
            return p;
        }
    }
    FieldValidation::ok()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldKind {
    Text,
    Number,
    Email,
}

#[derive(Debug, Clone, Copy)]
pub struct FieldInput<'a> {
    pub value: &'a str,
    pub required: bool,
    pub kind: FieldKind,
    pub min: Option<&'a str>,
    pub max: Option<&'a str>,
    pub pattern: Option<&'a str>,
    pub pattern_error: Option<&'a str>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_and_number_bounds() {
        assert!(!validate_required("  ").valid);
        assert!(validate_required("x").valid);
        assert!(!validate_number("abc", None, None).valid);
        assert!(!validate_number("1", Some("5"), None).valid);
        assert!(!validate_number("10", None, Some("5")).valid);
        assert!(validate_number("7", Some("1"), Some("10")).valid);
    }

    #[test]
    fn email_validation() {
        assert!(validate_email("a@b.co").valid);
        assert!(!validate_email("bad").valid);
        assert!(!validate_email("@b.co").valid);
    }

    #[test]
    fn validate_field_combined() {
        let bad = validate_field(FieldInput {
            value: "",
            required: true,
            kind: FieldKind::Text,
            min: None,
            max: None,
            pattern: None,
            pattern_error: None,
        });
        assert!(!bad.valid);

        let ok = validate_field(FieldInput {
            value: "user@example.com",
            required: true,
            kind: FieldKind::Email,
            min: None,
            max: None,
            pattern: None,
            pattern_error: None,
        });
        assert!(ok.valid);
    }
}
