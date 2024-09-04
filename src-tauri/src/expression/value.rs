pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    Column(Box<Value>),
    Null,
}

pub enum ValueType {
    Number,
    String,
    Boolean,
    Array,
    Null,
}

impl Value {
    pub fn value_type(&self) -> ValueType {
        match self {
            Value::Number(_) => ValueType::Number,
            Value::String(_) => ValueType::String,
            Value::Boolean(_) => ValueType::Boolean,
            Value::Array(_) => ValueType::Array,
            Value::Column(t) => t.value_type(),
            Value::Null => ValueType::Null,
        }
    }
}
