#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dec {
    mantissa: i128,
}

impl Dec {
	pub const fn zero() -> Self {
		Dec { mantissa: 0 }
	}

	pub const fn one() -> Self {
		Dec { mantissa: 1_000_000_000i128 }
	}

	pub fn from_i64(n: i64) -> Dec {
        Dec {
            mantissa: (n as i128) * 1_000_000_000i128,
        }
    }

	pub fn is_zero(&self) -> bool {
		self.mantissa == 0
	}
}
