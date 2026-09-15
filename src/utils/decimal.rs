use std::{
    fmt::{self, Display, Formatter}, ops::{Add, Neg, Sub}, str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dec {
    mantissa: i128,
}

impl Dec {
    pub const fn zero() -> Self {
        Dec { mantissa: 0 }
    }

    pub const fn one() -> Self {
        Dec {
            mantissa: 1_000_000_000i128,
        }
    }

    pub fn from_i64(n: i64) -> Dec {
        Dec {
            mantissa: (n as i128) * 1_000_000_000i128,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.mantissa == 0
    }

    pub fn checked_add(self, other: Dec) -> Option<Dec> {
        self.mantissa.checked_add(other.mantissa).map(|m| Dec { mantissa: m })
    }

    pub fn checked_sub(self, other: Dec) -> Option<Dec> {
        self.mantissa.checked_sub(other.mantissa).map(|m| Dec { mantissa: m })
    }
}

impl Neg for Dec {
    type Output = Dec;

    fn neg(self) -> Self::Output {
        Dec {
            mantissa: -self.mantissa,
        }
    }
}

impl Add for Dec {
    type Output = Dec;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs).expect("Overflow in addition")
    }
}

impl Sub for Dec {
    type Output = Dec;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs).expect("Overflow in substraction")
    }
}

impl FromStr for Dec {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let neg = s.starts_with('-');
        let s2 = s
            .strip_prefix('-')
            .or_else(|| s.strip_prefix('+'))
            .unwrap_or(s);

        let (int_part, frac_part) = match s2.split_once('.') {
            Some((i, f)) => (i, f),
            None => (s2, ""),
        };
        let int_part = if int_part.is_empty() { "0" } else { int_part };

        let int_val: i128 = int_part
            .parse()
            .map_err(|_| format!("Invalid number: '{}'", s))?;

        let scale = 9 as usize;
        let frac_digits: String = if frac_part.len() >= scale {
            frac_part[..scale].to_string()
        } else {
            format!("{:0<width$}", frac_part, width = scale)
        };
        let frac_val: i128 = if frac_digits.is_empty() {
            0
        } else {
            frac_digits
                .parse()
                .map_err(|_| format!("Invalid number: '{}'", s))?
        };

        let mantissa = int_val * 1_000_000_000i128 + frac_val;
        Ok(Dec {
            mantissa: if neg { -mantissa } else { mantissa },
        })
    }
}

impl Display for Dec {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let neg = self.mantissa < 0;
        let m = self.mantissa.unsigned_abs();
        let scale = 9 as usize;
        let s = m.to_string();
        let s = if s.len() <= scale {
            format!("{}{}", "0".repeat(scale - s.len() + 1), s)
        } else {
            s
        };
        let split_at = s.len() - scale;
        let (int_part, frac_part) = s.split_at(split_at);
        let frac_trimmed = frac_part.trim_end_matches('0');

        if neg && m != 0 {
            write!(f, "-")?;
        }
        write!(f, "{}", int_part)?;
        if !frac_trimmed.is_empty() {
            write!(f, ".{}", frac_trimmed)?;
        }
        Ok(())
    }
}
