pub enum Value {
	Byte(u8),
	Integer(Integer),
	Float(Float),
	Vector,
	Name(String),
}

pub enum Integer {
	U8(u8),
	U16(u16),
	U32(u32),
	U64(u64),
	S8(i8),
	S16(i16),
	S32(i32),
	S64(i64),
	I8(u8),
	I16(u16),
	I32(u32),
	I64(u64),
}

pub enum Float {
	F32(f32),
	F64(f64),
}
