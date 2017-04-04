use std::fmt;

///Main enum that represents a dynamic value
///###Example
///
///```
/// extern crate apple;
///let f = apple::val::Val::Int(8);
///```
pub enum Val {
	Int(i32),
	Str(String),
	Bool(bool),
	List(Vec<Val>),
	Null
}

//int operations, arithmetic

impl Val {
///Function that formats a `Val` into a String.
///###Example
///
///```
/// extern crate apple;
///let f = apple::val::Val::Int(8);
///println!("The format is {}", Val::repr(f));
///```
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

	pub fn from_ivec(lst:Vec<i32>) -> Val {
		let mut list:Vec<Val> = Vec::new();
		for elem in lst {
			list.push(Val::Int(elem))
		}
		Val::List(list)
	}
///Function that returns the corresponding i32 value of a `Val`.
///If the Val is not an int, returns 0
///###Example
///
///```
/// extern crate apple;
///let f = apple::val::Val::Int(8);
///println!("The int is {}", f.int());
///```
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

	pub fn bool(self) -> bool {
		match self {
			Val::Int(i) => i != 0,
			Val::Str(s) => s.len() != 0,
			Val::Bool(b) => b,
			Val::List(l) => l.len() != 0,
			_ => false
		}
	}

	pub fn is_int(self) -> bool {
		match self {
			Val::Int(_) => true,
			_ => false
		}
	}
///This function either adds integers or concats string `Val`s.
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

	pub fn pop(self) -> Val {
		match self {
			Val::List(mut l) => {
				match l.pop() {
					Some(res) => res,
					None => Val::Null
				}
			},
			_ => Val::Null
		}
	}

	pub fn get(self, other:usize) -> Val {
		let d = other;
		match self {
			Val::Str(s) => match s.chars().nth(d) {
				Some(chr) => Val::Str(chr.to_string()),
				None => Val::Null
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