use crate::ast::{Type, Value};
use crate::error::ProtlinError;

pub fn check_type(value: &Value, expected_type: &Type) -> Result<(), ProtlinError> {
    match (value, expected_type) {
        (Value::Integer(_), Type::Int) => Ok(()),
        (Value::Decimal(_), Type::Float) => Ok(()),
        (Value::String(_), Type::String) => Ok(()),
        (Value::Boolean(_), Type::Bool) => Ok(()),
        (Value::Void, Type::Void) => Ok(()),
        (_, Type::Any) => Ok(()),
        (Value::List(items), Type::List(inner_type)) => {
            for item in items {
                check_type(item, inner_type)?;
            }
            Ok(())
        }
        (Value::Tuple(items), Type::Tuple(types)) => {
            if items.len() != types.len() {
                return Err(ProtlinError::TypeError(
                    format!("Tuple length mismatch: expected {}, got {}", types.len(), items.len())
                ));
            }
            for (item, ty) in items.iter().zip(types.iter()) {
                check_type(item, ty)?;
            }
            Ok(())
        }
        _ => Err(ProtlinError::TypeError(
            format!("Type mismatch: expected {:?}, got {:?}", expected_type, value)
        )),
    }
}

pub fn infer_type(value: &Value) -> Type {
    match value {
        Value::Integer(_) => Type::Int,
        Value::Decimal(_) => Type::Float,
        Value::String(_) => Type::String,
        Value::Boolean(_) => Type::Bool,
        Value::Void => Type::Void,
        Value::Null => Type::Any,
        Value::List(items) => {
            if items.is_empty() {
                Type::List(Box::new(Type::Any))
            } else {
                Type::List(Box::new(infer_type(&items[0])))
            }
        }
        Value::Tuple(items) => {
            Type::Tuple(items.iter().map(infer_type).collect())
        }
        Value::Set(items) => {
            if items.is_empty() {
                Type::Set(Box::new(Type::Any))
            } else {
                Type::Set(Box::new(infer_type(&items[0])))
            }
        }
        Value::Dict(_) => Type::Dict(Box::new(Type::String), Box::new(Type::Any)),
        Value::Function { .. } | Value::NativeFunction { .. } => {
            Type::Function {
                parameters: vec![],
                return_type: Box::new(Type::Any),
            }
        }
        Value::Object { class_name, .. } => Type::Custom(class_name.clone()),
        Value::Range { .. } => Type::Custom("Range".to_string()),
    }
}

pub fn coerce_to_int(value: &Value) -> Result<i64, ProtlinError> {
    match value {
        Value::Integer(n) => Ok(*n),
        Value::Decimal(f) => Ok(*f as i64),
        Value::Boolean(b) => Ok(if *b { 1 } else { 0 }),
        Value::String(s) => s.parse::<i64>()
            .map_err(|_| ProtlinError::TypeError(format!("Cannot convert '{}' to integer", s))),
        _ => Err(ProtlinError::TypeError(format!("Cannot convert {:?} to integer", value))),
    }
}

pub fn coerce_to_float(value: &Value) -> Result<f64, ProtlinError> {
    match value {
        Value::Integer(n) => Ok(*n as f64),
        Value::Decimal(f) => Ok(*f),
        Value::Boolean(b) => Ok(if *b { 1.0 } else { 0.0 }),
        Value::String(s) => s.parse::<f64>()
            .map_err(|_| ProtlinError::TypeError(format!("Cannot convert '{}' to float", s))),
        _ => Err(ProtlinError::TypeError(format!("Cannot convert {:?} to float", value))),
    }
}

pub fn coerce_to_string(value: &Value) -> String {
    format!("{}", value)
}

pub fn coerce_to_bool(value: &Value) -> bool {
    match value {
        Value::Boolean(b) => *b,
        Value::Integer(n) => *n != 0,
        Value::Decimal(f) => *f != 0.0,
        Value::String(s) => !s.is_empty(),
        Value::Null | Value::Void => false,
        Value::List(items) => !items.is_empty(),
        Value::Dict(map) => !map.is_empty(),
        Value::Set(items) => !items.is_empty(),
        Value::Tuple(items) => !items.is_empty(),
        _ => true,
    }
}
