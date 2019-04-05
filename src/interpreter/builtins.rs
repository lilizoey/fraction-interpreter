use super::ast::{Value, Builtin, RuntimeError};
use super::data_structures as ds;
use rpds::List;

fn add2<'a>(arg1: Value<'a>, arg2: &Value<'a>) -> Result<Value<'a>, RuntimeError> {
    if let (Value::Number(n1), Value::Number(n2)) = (arg1, arg2) {
        Ok(Value::Number(n1 + *n2))
    } else {
        Err(RuntimeError::TypeError("Must add two numbers".to_owned()))
    }
}

fn add<'a>(args: &List<Value<'a>>) -> Result<Value<'a>, RuntimeError> {
    args.iter().fold(Ok(Value::Number(ds::Number::Integer(0))), |x, y| add2(x?, y))
}

const ADD: Builtin<'static> = Builtin {
    func: &add,
};