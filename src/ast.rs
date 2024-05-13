#[derive(Debug)]
pub enum Value {
    String(String),
    Object(Object),
    Array(Vec<Value>),
    Number(f32),
    Boolean(bool),
    Null,
}

#[derive(Debug)]
pub struct Object {
    pub values: Vec<(Value, Value)>,
}
