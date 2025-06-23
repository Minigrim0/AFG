use bevy_egui::egui::{self, text::LayoutJob, FontId, TextFormat};
use std::collections::HashSet;

use super::colors::AfgSyntaxColors;

// AFG language keywords
fn get_afg_keywords() -> HashSet<&'static str> {
    [
        "fn", "set", "if", "while", "loop", "call", "return", "print",
    ]
    .iter()
    .cloned()
    .collect()
}

// AFG operators
fn get_afg_operators() -> HashSet<&'static str> {
    [
        "+", "-", "*", "/", "%", "<", ">", "<=", ">=", "==", "!=", "=",
    ]
    .iter()
    .cloned()
    .collect()
}

// System variables that start with $
fn is_system_variable(word: &str) -> bool {
    word.starts_with('$')
}

// Check if a character is valid for identifiers
fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$'
}

// Check if a character is a digit or negative sign followed by digit
fn is_number_start(chars: &[char], pos: usize) -> bool {
    if pos >= chars.len() {
        return false;
    }

    let c = chars[pos];
    if c.is_ascii_digit() {
        return true;
    }

    // Check for negative numbers
    if c == '-' && pos + 1 < chars.len() && chars[pos + 1].is_ascii_digit() {
        // Make sure it's not part of an operator like -=
        if pos == 0 || !chars[pos - 1].is_alphanumeric() {
            return true;
        }
    }

    false
}

pub fn highlight_afg_syntax(
    ui: &egui::Ui,
    text: &str,
    wrap_width: f32,
) -> std::sync::Arc<egui::Galley> {
    let keywords = get_afg_keywords();
    let operators = get_afg_operators();
    let colors = AfgSyntaxColors::default();

    let mut job = LayoutJob::default();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let start_i = i;

        // Handle comments
        if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '/' {
            // Find end of line
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }

            let comment_text: String = chars[start_i..i].iter().collect();
            job.append(
                &comment_text,
                0.0,
                TextFormat {
                    font_id: FontId::monospace(12.0),
                    color: colors.comment,
                    italics: true,
                    ..Default::default()
                },
            );
            continue;
        }

        // Handle numbers (including negative numbers)
        if is_number_start(&chars, i) {
            if chars[i] == '-' {
                i += 1; // Skip the negative sign
            }

            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }

            let number_text: String = chars[start_i..i].iter().collect();
            job.append(
                &number_text,
                0.0,
                TextFormat {
                    font_id: FontId::monospace(12.0),
                    color: colors.number,
                    ..Default::default()
                },
            );
            continue;
        }

        // Handle identifiers (keywords, variables, function names)
        if chars[i].is_alphabetic() || chars[i] == '_' || chars[i] == '$' {
            while i < chars.len() && is_identifier_char(chars[i]) {
                i += 1;
            }

            let word: String = chars[start_i..i].iter().collect();

            let color = if keywords.contains(word.as_str()) {
                colors.keyword
            } else if is_system_variable(&word) {
                colors.system_var
            } else {
                // Check if this might be a function name (followed by '(')
                let mut j = i;
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }
                if j < chars.len() && chars[j] == '(' {
                    colors.function_name
                } else {
                    colors.variable
                }
            };

            let is_bold = keywords.contains(word.as_str());

            job.append(
                &word,
                0.0,
                TextFormat {
                    font_id: FontId::monospace(12.0),
                    color,
                    ..Default::default()
                },
            );
            continue;
        }

        // Handle operators
        let mut operator_found = false;
        for &op in &operators {
            if i + op.len() <= chars.len() {
                let potential_op: String = chars[i..i + op.len()].iter().collect();
                if potential_op == op {
                    job.append(
                        op,
                        0.0,
                        TextFormat {
                            font_id: FontId::monospace(12.0),
                            color: colors.operator,
                            ..Default::default()
                        },
                    );
                    i += op.len();
                    operator_found = true;
                    break;
                }
            }
        }

        if operator_found {
            continue;
        }

        // Handle brackets and braces with special coloring
        if "(){}[]".contains(chars[i]) {
            job.append(
                &chars[i].to_string(),
                0.0,
                TextFormat {
                    font_id: FontId::monospace(12.0),
                    color: colors.operator,
                    ..Default::default()
                },
            );
            i += 1;
            continue;
        }

        // Handle everything else (whitespace, punctuation, etc.)
        let ch = chars[i].to_string();
        job.append(
            &ch,
            0.0,
            TextFormat {
                font_id: FontId::monospace(12.0),
                color: colors.default,
                ..Default::default()
            },
        );
        i += 1;
    }

    job.wrap.max_width = wrap_width;
    ui.fonts(|f| f.layout_job(job))
}
