//main file for Val enum

pub enum Val {
	Int(i32),
	Str(String),
	Bool(bool),
	List(Vec<Val>),
	Null
}

//number operations, arithmetic

impl Val {

	pub fn int(self) -> i32 {
		match self {
			Val::Int(i) => i,
			_ => 0
		}
	}

	pub fn plus(self, other:Val) -> Val {
		match self {
			Val::Int(i) => match other {
				Val::Int(j) => Val::Int(i + j),
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
}