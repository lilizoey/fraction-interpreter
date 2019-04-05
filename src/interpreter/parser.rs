use pest::{Parser, error::Error};
use super::ast;

#[derive(Parser)]
#[grammar = "interpreter/calculator.pest"]
struct FractionParser;

fn test() {
    
    unimplemented!();
}

fn parseFractionString<'a>(parent: Option<Box<&'a ast::Environment<'a>>>, file: &str) -> Result<ast::Expression<'a>, Error<Rule>> {
    
}