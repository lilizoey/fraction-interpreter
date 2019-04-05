use std::collections::{HashMap};
use rpds::List;

#[derive(Debug)]
struct ParseError {
    explanation: String,
    location: usize,
}

#[derive(Debug)]
enum RuntimeError {
    UndefinedVariable(String),
    NotAFunction(String),
    IncorrectNumberOfArgs(usize, usize), // got, needed
    TypeError(String),
}

struct Environment<'a> {
    parent: Box<Option<&'a Environment<'a>>>,
    storage: HashMap<String, Value<'a>>,
}

impl<'a> Environment<'a> {
    fn new(parent: Option<&'a Environment<'a>>, defines: Vec<(String, Value<'a>)>) -> Self {
        let mut m: HashMap<String, Value<'a>> = HashMap::new();

        for (name, val) in defines {
            m.insert(name, val);
        }

        Environment {
            parent: Box::new(parent),
            storage: m,
        }
    }

    fn get(&self, name: String) -> Option<&Value<'a>> {
        if self.storage.contains_key(&name) {
            self.storage.get(&name)
        } else if self.parent.is_some() {
            self.parent.unwrap().get(name)
        } else {
            None
        }
    }

    fn get_runtime(&self, name: String) -> Result<&Value<'a>, RuntimeError> {
        match self.get(name) {
            Some(v) => Ok(v),
            None => Err(RuntimeError::UndefinedVariable(name)),
        }
    }

    fn evaluate(&self, expr: &'a Expression<'a>) -> Result<Value<'a>, RuntimeError> {
        // Gotta fight the borrow checker a bit here
        match expr {
            Expression::Value(val) => val.evaluate(Some(self)),
            Expression::Application(operator, operands) => operator.call(Some(self), operands),
        }
    }
}

struct Function<'a> {
    argnames: Vec<String>,
    expr: Box<&'a Expression<'a>>,
}

impl<'a> Function<'a> {
    fn call(&'a self, env: Option<&'a Environment<'a>>, operands: &Vec<Box<Expression<'a>>>) -> Result<Value<'a>, RuntimeError> {
        if self.argnames.len() != operands.len() {
            return Err(RuntimeError::IncorrectNumberOfArgs(operands.len(), self.argnames.len()));
        }

        let mut defines = vec!();

        for i in 0..operands.len() {
            defines.push((self.argnames[i].clone(), operands[i].evaluate(env)?));
        }

        let env = Environment::new(env, defines);

        env.evaluate(&self.expr)
    }
}

#[derive(Clone)]
enum Number {
    Float(f64),
    Integer(i128),
}

#[derive(Clone)]
enum Value<'a> {
    Number(Number),
    Glyph(char),
    Variable(String),
    List(List<Box<Expression<'a>>>),
    Func(&'a Function<'a>),
    Builtin(&'a Box<Fn(Option<&Environment>, &Vec<Box<Expression<'a>>>) -> Result<Value<'a>, RuntimeError>>),
}

impl<'a> Value<'a> {
    fn evaluate(&self, env: Option<&'a Environment<'a>>) -> Result<Value<'a>, RuntimeError> {
        match self {
            Value::Variable(s) => {
                if let Some(env) = env {
                    env.get_runtime(s.to_string()).map(|s| s.to_owned())
                } else {
                    Err(RuntimeError::UndefinedVariable(s.to_string()))
                }
            },
            other => Ok(*other), 
        }
    }
}

enum Expression<'a> {
    Value(Value<'a>),
    Application(Box<Expression<'a>>, Vec<Box<Expression<'a>>>),
}

impl<'a> Expression<'a> {
    fn evaluate(&self, env: Option<&'a Environment<'a>>) -> Result<Value<'a>, RuntimeError> {
        match self {
            Expression::Application(operator, operands) => operator.call(env, operands),
            Expression::Value(value) => Ok(value.clone()),
        }
    }

    fn call(&self, env: Option<&'a Environment<'a>>, operands: &Vec<Box<Expression<'a>>>) -> Result<Value<'a>, RuntimeError> {
        if let Expression::Value(val) = self {
            match val {
                Value::Func(f) => f.call(env, operands),
                Value::Builtin(f) => f(env, operands),
                _ => Err(RuntimeError::NotAFunction("".to_owned())),
            }
        } else {
            Err(RuntimeError::NotAFunction("".to_owned()))
        }
    }
}

// Defining some builtins
// Builtin(&'a Box<Fn(Option<&Environment>, &Vec<Box<Expression<'a>>>) -> Result<Value<'a>, RuntimeError>>),

fn check_arity(actual: usize, expected: usize) -> Result<(), RuntimeError> {
    if expected != actual {
        Err(RuntimeError::IncorrectNumberOfArgs(actual, expected))
    } else {
        Ok(())
    }
}

fn add<'a>(env: Option<&'a Environment<'a>>, operands: &Vec<Box<Expression<'a>>>) -> Result<Value<'a>, RuntimeError> {
    check_arity(operands.len(), 2)?;

    let fst = operands[0].evaluate(env)?;
    let snd = operands[1].evaluate(env)?;

    match fst {
        Value::Float(f) => add_float(f, snd),
        Value::Integer(i) => add_int(i, snd),
        other => Err(RuntimeError::TypeError("Must add two numbers".to_owned())),
    }
}

fn add_float<'a>(f: f64, snd: Value) -> Result<Value<'a>, RuntimeError> {
    match snd {
        Value::Float(f2) => Ok(Value::Float(f + f2)),
        Value::Integer(i) => Ok(Value::Float(f + (i as f64))),
        other => Err(RuntimeError::TypeError("Must add two numbers".to_owned())),
    }
}

fn add_int<'a>(i: i128, snd: Value) -> Result<Value<'a>, RuntimeError> {
    match snd {
        Value::Float(f) => Ok(Value::Float(f + (i as f64))),
        Value::Integer(i2) => Ok(Value::Integer(i + i2)),
        other => Err(RuntimeError::TypeError("Must add two numbers".to_owned())),
    }
}

fn negate<'a>(env: Option<&'a Environment<'a>>, operands: &Vec<Box<Expression<'a>>>) -> Result<Value<'a>, RuntimeError> {
    check_arity(operands.len(), 1)?;

    match operands[0].evaluate(env)? {
        Value::Float(f) => Ok(Value::Float(-f)),
        Value::Integer(i) => Ok(Value::Integer(-i)),
        other => Err(RuntimeError::TypeError("Must negate a number".to_owned())),
    }
}

fn sub<'a>(env: Option<&'a Environment<'a>>, operands: &Vec<Box<Expression<'a>>>) -> Result<Value<'a>, RuntimeError> {
    check_arity(operands.len(), 2)?;

    let new_ops = vec!(operands[0], Box::new(Expression::Value(negate(env, &vec!(operands[1]))?)));
    add(env, &new_ops)
}

