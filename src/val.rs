//main file for Val enum

pub enum Val {
	Int(i32),
	Str(String),
	Bool(bool),
	List(Vec<Val>),
	Null
}

//int operations, arithmetic

impl Val {

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
}