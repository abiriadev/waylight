pub enum Type {
	ValueType(ValueType),
	ResultType(ResultType),
	FunctionType(ResultType, ResultType),
}

pub enum ValueType {
	NumberType(NumberType),
	VectorType,
	ReferenceType(ReferenceType),
}

pub enum NumberType {
	I32,
	I64,
	F32,
	F64,
}

pub enum ReferenceType {
	FuncRef,
	ExternRef,
}

pub struct ResultType(Vec<ValueType>);
