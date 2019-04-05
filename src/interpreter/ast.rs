// Alternate implementation of ast?
use std::collections::HashMap;
use super::data_structures::{Name, Number};
use rpds::List;

pub enum RuntimeError {
    UndefinedVariable(Name),
    // expected, actual
    IncorrectNumberOfArgs(usize, usize),
    CannotCallValue(String),
}

/// An environment usually has a parent environment, unless it is
/// the top level environment.
struct Environment<'a> {
    parent: Option<Box<&'a Environment<'a>>>,
    mappings: HashMap<Name, Value<'a>>,
}

impl<'a> Environment<'a> {
    fn new(parent: Option<Box<&'a Environment<'a>>>, defines: Vec<(&Name, &Value<'a>)>) -> Self {
        let mut map = HashMap::new();
        for (name, val) in defines {
            map.insert(name.to_owned(), val.clone());
        }

        Environment {
            parent: parent,
            mappings: map,
        }
    }

    fn get(&self, name: &Name) -> Result<Value<'a>, RuntimeError> {
        if self.mappings.contains_key(name) {
            return Ok(self.mappings[name].clone());
        }

        if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            Err(RuntimeError::UndefinedVariable(name.to_owned()))
        }
    }

    fn evaluate(&self, expr: &'a Expression<'a>) -> Result<Value<'a>, RuntimeError> {
        match expr {
            Expression::Atomic(val) => Ok(val.clone()),
            Expression::Variable(name) => self.get(name),
            Expression::Application(func, args) => func.try_call(args).map(|v| v.clone()),
        }

    }
}

/// A closure holds a reference to its enclosing environment, and
/// the expression that is run when it is evaluated.
/// It also has a list of all the names of the arguments, and 
/// whether it's variadic or not, meaning it looks like 
/// `(a, b, c, x.) -> ...`
#[derive(Clone)]
struct Closure<'a> {    
    env: Box<&'a Environment<'a>>,
    expr: Box<&'a Expression<'a>>,
    argnames: Vec<Name>,
    variadic: bool,
}

impl<'a> Closure<'a> {
    fn call(&'a self, args: &'a List<Value<'a>>) -> Result<Value<'a>, RuntimeError> {
        if self.argnames.len() > args.len() ||
           (self.argnames.len() < args.len() && !self.variadic) {
            return Err(RuntimeError::IncorrectNumberOfArgs(self.argnames.len(), args.len()));
        }

        let defines: Vec<(&Name, &Value<'a>)> = self.argnames.iter().zip(args.into_iter()).collect();

        let new_env = Environment::new(Some(Box::new(&self.env)), defines);

        new_env.evaluate(&self.expr)
    }
}

#[derive(Clone)]
struct Builtin<'a> {
    func: Box<&'a Fn (&List<Value<'a>>) -> Result<Value<'a>, RuntimeError>>,
}

/// A value is something that can be passed around and stored in 
/// variables.
#[derive(Clone)]
enum Value<'a> {
    Number(Number),
    // A glyph is basically a character. A string is a list of 
    // glyphs.
    Glyph(char),
    List(List<Value<'a>>),
    Function(Closure<'a>),
    Builtin(Builtin<'a>),
}

impl<'a> Value<'a> {
    fn try_call(&'a self, args: &'a List<Value<'a>>) -> Result<Value<'a>, RuntimeError> {
        match self {
            Value::Function(func) => func.call(args),
            Value::Builtin(func) => (func.func)(args),
            _ => Err(RuntimeError::CannotCallValue("must call a function".to_owned())),
        }
    }
}

enum Expression<'a> {
    Atomic(Value<'a>),
    Variable(Name),
    // First one is the function, if it's not a function it will 
    // give a runtime exception.
    Application(Value<'a>, List<Value<'a>>),
}
