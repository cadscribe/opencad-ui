//! Number and length input: arithmetic (`+ − × ÷`, parentheses), units
//! (mm, cm, dm, m), decimal comma or point; formatting in the language's
//! decimal separator. Used by the number and length fields.

use crate::i18n::Lang;

/// A length unit; values are kept in millimetres.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Unit {
    Mm,
    Cm,
    Dm,
    #[default]
    M,
}

impl Unit {
    pub const ALL: [Unit; 4] = [Unit::Mm, Unit::Cm, Unit::Dm, Unit::M];

    /// Millimetres per unit.
    pub fn mm(self) -> f64 {
        match self {
            Unit::Mm => 1.0,
            Unit::Cm => 10.0,
            Unit::Dm => 100.0,
            Unit::M => 1000.0,
        }
    }

    pub fn symbol(self) -> &'static str {
        match self {
            Unit::Mm => "mm",
            Unit::Cm => "cm",
            Unit::Dm => "dm",
            Unit::M => "m",
        }
    }

    /// Decimals shown by default (m: 3 like "10,240 m").
    pub fn decimals(self) -> usize {
        match self {
            Unit::Mm => 0,
            Unit::Cm => 1,
            Unit::Dm => 2,
            Unit::M => 3,
        }
    }

    fn from_symbol(s: &str) -> Option<Unit> {
        Unit::ALL.into_iter().find(|u| u.symbol() == s)
    }
}

/// Why an input could not be read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputError {
    Empty,
    /// Something that is not a number, unit or operator, at a char index.
    Unexpected(usize),
    /// Division by zero.
    DivisionByZero,
    /// Two lengths multiplied or divided ("2 m × 3 m").
    LengthTimesLength,
    /// A unit in a field without units.
    UnitNotAllowed,
}

#[derive(Debug, Clone, Copy)]
struct Value {
    v: f64,
    /// Whether `v` is already in millimetres (a unit was written).
    length: bool,
}

struct Parser<'a> {
    chars: Vec<char>,
    i: usize,
    units: bool,
    _src: &'a str,
}

impl Parser<'_> {
    fn ws(&mut self) {
        while self.i < self.chars.len() && self.chars[self.i].is_whitespace() {
            self.i += 1;
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.ws();
        self.chars.get(self.i).copied()
    }

    fn expr(&mut self) -> Result<Value, InputError> {
        let mut a = self.term()?;
        loop {
            match self.peek() {
                Some('+') => {
                    self.i += 1;
                    let b = self.term()?;
                    a = add(a, b, 1.0);
                }
                Some('-' | '−') => {
                    self.i += 1;
                    let b = self.term()?;
                    a = add(a, b, -1.0);
                }
                _ => return Ok(a),
            }
        }
    }

    fn term(&mut self) -> Result<Value, InputError> {
        let mut a = self.factor()?;
        loop {
            match self.peek() {
                Some('*' | '×' | '·') => {
                    self.i += 1;
                    let b = self.factor()?;
                    if a.length && b.length {
                        return Err(InputError::LengthTimesLength);
                    }
                    a = Value {
                        v: a.v * b.v,
                        length: a.length || b.length,
                    };
                }
                Some('/' | '÷' | ':') => {
                    self.i += 1;
                    let b = self.factor()?;
                    if b.length {
                        return Err(InputError::LengthTimesLength);
                    }
                    if b.v == 0.0 {
                        return Err(InputError::DivisionByZero);
                    }
                    a = Value {
                        v: a.v / b.v,
                        length: a.length,
                    };
                }
                _ => return Ok(a),
            }
        }
    }

    fn factor(&mut self) -> Result<Value, InputError> {
        match self.peek() {
            Some('-' | '−') => {
                self.i += 1;
                let v = self.factor()?;
                Ok(Value { v: -v.v, ..v })
            }
            Some('+') => {
                self.i += 1;
                self.factor()
            }
            Some('(') => {
                self.i += 1;
                let v = self.expr()?;
                if self.peek() != Some(')') {
                    return Err(InputError::Unexpected(self.i));
                }
                self.i += 1;
                self.unit(v)
            }
            Some(c) if c.is_ascii_digit() || c == '.' || c == ',' => {
                let start = self.i;
                let mut s = String::new();
                while let Some(&c) = self.chars.get(self.i) {
                    if c.is_ascii_digit() {
                        s.push(c);
                    } else if c == '.' || c == ',' {
                        s.push('.');
                    } else {
                        break;
                    }
                    self.i += 1;
                }
                let v: f64 = s.parse().map_err(|_| InputError::Unexpected(start))?;
                self.unit(Value { v, length: false })
            }
            Some(_) => Err(InputError::Unexpected(self.i)),
            None => Err(InputError::Unexpected(self.i)),
        }
    }

    fn unit(&mut self, v: Value) -> Result<Value, InputError> {
        self.ws();
        let start = self.i;
        let mut s = String::new();
        while let Some(&c) = self.chars.get(self.i) {
            if c.is_ascii_alphabetic() {
                s.push(c.to_ascii_lowercase());
                self.i += 1;
            } else {
                break;
            }
        }
        if s.is_empty() {
            return Ok(v);
        }
        if !self.units {
            return Err(InputError::UnitNotAllowed);
        }
        match Unit::from_symbol(&s) {
            Some(u) if !v.length => Ok(Value {
                v: v.v * u.mm(),
                length: true,
            }),
            _ => Err(InputError::Unexpected(start)),
        }
    }
}

fn add(a: Value, b: Value, sign: f64) -> Value {
    // Mixed: a plain number next to a length is in the default unit;
    // resolved at the end, so keep a flag per side by scaling later.
    match (a.length, b.length) {
        (true, true) | (false, false) => Value {
            v: a.v + sign * b.v,
            length: a.length,
        },
        // Mark mixed sums by converting the plain side when the default
        // unit is known: see `parse_length`, which pre-scales plain
        // numbers. Here both are already in the same unit.
        _ => Value {
            v: a.v + sign * b.v,
            length: true,
        },
    }
}

fn parse(input: &str, units: bool, default_mm: f64) -> Result<f64, InputError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(InputError::Empty);
    }
    // Plain numbers in sums with lengths are in the default unit: scale
    // every plain number that is directly followed by `+`/`-` context by
    // parsing twice: first find out whether any unit appears at all.
    let mut p = Parser {
        chars: trimmed.chars().collect(),
        i: 0,
        units,
        _src: trimmed,
    };
    let any_unit = units && trimmed.chars().any(|c| c.is_ascii_alphabetic());
    let v = if any_unit {
        // Rewrite plain operands of + and - into the default unit.
        let mut q = Parser {
            chars: scale_plain_terms(trimmed, default_mm),
            i: 0,
            units,
            _src: trimmed,
        };
        let v = q.expr()?;
        if q.peek().is_some() {
            return Err(InputError::Unexpected(q.i));
        }
        v
    } else {
        let v = p.expr()?;
        if p.peek().is_some() {
            return Err(InputError::Unexpected(p.i));
        }
        v
    };
    Ok(if v.length { v.v } else { v.v * default_mm })
}

/// Gives every additive term without a unit the unit "mm" after scaling
/// it into millimetres, so "1,2 m + 30" with the default unit cm reads
/// as 1200 mm + 300 mm. Multiplicative factors stay plain.
fn scale_plain_terms(s: &str, default_mm: f64) -> Vec<char> {
    // Split at top-level + and - (not unary), keep the operators.
    let chars: Vec<char> = s.chars().collect();
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut term = Vec::new();
    let mut prev_operand = false;
    let flush = |term: &mut Vec<char>, out: &mut Vec<char>| {
        let t: String = term.iter().collect();
        let has_unit = t.chars().any(|c| c.is_ascii_alphabetic());
        if !has_unit && !t.trim().is_empty() {
            out.push('(');
            out.extend(t.chars());
            out.extend(format!(")*{default_mm}mm").chars());
        } else {
            out.extend(t.chars());
        }
        term.clear();
    };
    for &c in &chars {
        match c {
            '(' => {
                depth += 1;
                term.push(c);
                prev_operand = false;
            }
            ')' => {
                depth -= 1;
                term.push(c);
                prev_operand = true;
            }
            '+' | '-' | '−' if depth == 0 && prev_operand => {
                flush(&mut term, &mut out);
                out.push(c);
                prev_operand = false;
            }
            c if c.is_whitespace() => term.push(c),
            _ => {
                term.push(c);
                prev_operand = !matches!(c, '*' | '×' | '·' | '/' | '÷' | ':');
            }
        }
    }
    flush(&mut term, &mut out);
    out
}

/// Reads a length in millimetres: "10,24", "10.24 m", "1 m + 24 cm",
/// "(2,5 + 1) × 2". Plain numbers are in `default`.
pub fn parse_length(input: &str, default: Unit) -> Result<f64, InputError> {
    parse(input, true, default.mm())
}

/// Reads a plain number with arithmetic ("12,5", "3 × 4 + 1").
pub fn parse_number(input: &str) -> Result<f64, InputError> {
    parse(input, false, 1.0)
}

/// Formats a number with `decimals` places in the language's decimal
/// separator and a real minus sign.
pub fn format_number(v: f64, decimals: usize, lang: Lang) -> String {
    let s = format!("{:.*}", decimals, v);
    let s = if s.starts_with('-') && s.trim_start_matches(['-', '0', '.']).is_empty() {
        // "-0.000" → "0.000"
        s[1..].to_owned()
    } else {
        s
    };
    s.replace('.', &lang.decimal().to_string())
        .replacen('-', "−", 1)
}

/// Formats a length in millimetres in `unit`: "10,240 m".
pub fn format_length(mm: f64, unit: Unit, lang: Lang) -> String {
    format!(
        "{} {}",
        format_number(mm / unit.mm(), unit.decimals(), lang),
        unit.symbol()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn near(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn plain_numbers_and_arithmetic() {
        assert!(near(parse_number("12,5").unwrap(), 12.5));
        assert!(near(parse_number("12.5").unwrap(), 12.5));
        assert!(near(parse_number("3 × 4 + 1").unwrap(), 13.0));
        assert!(near(parse_number("(1 + 2) * 3").unwrap(), 9.0));
        assert!(near(parse_number("10 / 4").unwrap(), 2.5));
        assert!(near(parse_number("−2 + 5").unwrap(), 3.0));
        assert!(near(parse_number("-(2 + 3)").unwrap(), -5.0));
        assert_eq!(parse_number(""), Err(InputError::Empty));
        assert_eq!(parse_number("1 / 0"), Err(InputError::DivisionByZero));
        assert!(matches!(
            parse_number("2 x"),
            Err(InputError::UnitNotAllowed)
        ));
        assert!(matches!(
            parse_number("2 +"),
            Err(InputError::Unexpected(_))
        ));
    }

    #[test]
    fn lengths_with_units() {
        assert!(near(parse_length("10,24", Unit::M).unwrap(), 10_240.0));
        assert!(near(parse_length("175", Unit::Mm).unwrap(), 175.0));
        assert!(near(parse_length("17,5 cm", Unit::Mm).unwrap(), 175.0));
        assert!(near(parse_length("1 m + 24 cm", Unit::Mm).unwrap(), 1240.0));
        // A plain term next to lengths is in the default unit.
        assert!(near(parse_length("1,2 m + 30", Unit::Cm).unwrap(), 1500.0));
        assert!(near(parse_length("2 × 1,5 m", Unit::Mm).unwrap(), 3000.0));
        assert!(near(
            parse_length("(2,5 + 1) × 2", Unit::M).unwrap(),
            7000.0
        ));
        assert!(near(parse_length("3 m / 2", Unit::Mm).unwrap(), 1500.0));
        assert_eq!(
            parse_length("2 m × 3 m", Unit::M),
            Err(InputError::LengthTimesLength)
        );
        assert!(parse_length("2 km", Unit::M).is_err());
    }

    #[test]
    fn formatting() {
        assert_eq!(format_length(10_240.0, Unit::M, Lang::De), "10,240 m");
        assert_eq!(format_length(10_240.0, Unit::M, Lang::En), "10.240 m");
        assert_eq!(format_length(175.0, Unit::Mm, Lang::De), "175 mm");
        assert_eq!(format_number(-0.08, 3, Lang::De), "−0,080");
        assert_eq!(format_number(-0.0001, 3, Lang::De), "0,000");
    }
}
