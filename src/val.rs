use std::fmt;

pub enum Val {
	Int(i32),
	Str(String),
	Bool(bool),
	List(Vec<Val>),
	Null
}

//int operations, arithmetic

impl Val {

	pub fn repr(value:&Val) -> String {
		match *value {
			Val::Int(i) => format!("{}", i),
			Val::Str(ref s) => format!("{}", s),
			Val::Bool(b) => format!("{}", b),
			Val::Null => "Null".to_string(),
			Val::List(ref l) => {
				let mut fmt_str = "[".to_string();
				for elem in l {
					fmt_str += Val::repr(elem).as_str();
					fmt_str += ",";
				}
				fmt_str += "]";
				fmt_str
			}
		}
	}

	pub fn from_str(s: &str) -> Val {
		Val::Str(String::from(s))
	}

	pub fn from_int(elem:i32) -> Val {
		Val::Int(elem)
	}

	pub fn int(self) -> i32 {
		match self {
			Val::Int(i) => i,
			_ => 0
		}
	}

	pub fn str(self) -> String {
		match self {
			Val::Str(s) => s,
			_ => "".to_string()
		}
	}

	pub fn is_int(self) -> bool {
		match self {
			Val::Int(_) => true,
			_ => false
		}
	}

	pub fn plus(self, other:Val) -> Val {
		match self {
			Val::Int(i) => match other {
				Val::Int(j) => Val::Int(i + j),
				_ => Val::Null
			},
			Val::Str(s) => match other {
				Val::Str(so) => Val::Str(s + &so),
				_ => Val::Null
			},
			_ => Val::Null
		}
	}

	pub fn sub(self, other:Val) -> Val {
		match self {
			Val::Int(i) => match other {
				Val::Int(j) => Val::Int(i - j),
				_ => Val::Null
			},
			_ => Val::Null
		}
	}

	pub fn mul(self, other:Val) -> Val {
		match self {
			Val::Int(i) => match other {
				Val::Int(j) => Val::Int(i * j),
				_ => Val::Null
			},
			_ => Val::Null
		}
	}

	pub fn div(self, other:Val) -> Val {
		match self {
			Val::Int(i) => match other {
				Val::Int(j) => Val::Int(i / j),
				_ => Val::Null
			},
			_ => Val::Null
		}
	}

	pub fn rem(self, other:Val) -> Val {
		match self {
			Val::Int(i) => match other {
				Val::Int(j) => Val::Int(i % j),
				_ => Val::Null
			},
			_ => Val::Null
		}
	}

	pub fn min(self, other:Val) -> Val {
		match self {
			Val::Int(i) => match other {
				Val::Int(j) => return if i <= j {Val::Int(i)} else {Val::Int(j)},
				_ => Val::Null
			},
			_ => Val::Null
		}
	}

	pub fn max(self, other:Val) -> Val {
		match self {
			Val::Int(i) => match other {
				Val::Int(j) => return if i >= j {Val::Int(i)} else {Val::Int(j)},
				_ => Val::Null
			},
			_ => Val::Null
		}
	}

	pub fn len(self) -> i32 {
		match self {
			Val::Int(i) => i,
			Val::Str(s) => s.len() as i32,
			Val::Bool(_) => 1,
			Val::List(l) => l.len() as i32,
			Val::Null => 0
		}
	}

	pub fn push(self, other:Val) -> Val {
		match self {
			Val::List(mut l) => {
				l.push(other);
				Val::List(l)
			},
			_ => Val::Null
		}
	}
}

impl fmt::Debug for Val {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", Val::repr(self))
	}
}

impl PartialEq for Val {
    fn eq(&self, other: &Val) -> bool {
        Val::repr(self) == Val::repr(other)
    }
}