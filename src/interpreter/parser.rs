use pest::{Parser, error::Error, iterators::Pair};
use super::ast;
use rpds::List;

#[derive(Parser)]
#[grammar = "interpreter/calculator.pest"]
pub struct FractionParser;

fn parse_fraction_string<'a>(file: &str) -> Result<ast::Program<'a>, Error<Rule>> {
    let parse = FractionParser::parse(Rule::program, file)?;
    let mut program = List::new();

    for pair in parse {
        let stmt = statement_parser(pair)?;
        program = program.push_front(stmt);        
    }

    Ok(program.reverse())
}

fn statement_parser<'a>(pair: Pair<Rule>) -> Result<ast::Statement<'a>, Error<Rule>> {
    assert!(pair.as_rule() == Rule::program);

    unimplemented!();
}

fn assignment_parser<'a>(pair: Pair<Rule>) -> Result<ast::Statement<'a>, Error<Rule>> {
    assert!(pair.as_rule() == Rule::assignment);
    let assignment_type = pair.into_inner().next().unwrap();

    match assignment_type.as_rule() {
        Rule::var_assign => unimplemented!(),
        Rule::func_assign => unimplemented!(),
        _ => unreachable!(),
    };

    unimplemented!();
}

fn expression_parser<'a>(pair: Pair<Rule>) -> Result<ast::Statement<'a>, Error<Rule>> {
    
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let parse = FractionParser::parse(Rule::program, "foo(a, 2)");
        println!("{:#?}", parse);
        let parse = FractionParser::parse(Rule::program, "a + b");
        println!("{:#?}", parse);
        let parse = FractionParser::parse(Rule::program, "'b");
        println!("{:#?}", parse);
        let parse = FractionParser::parse(Rule::program, "(a, b) -> a");
        println!("{:#?}", parse);
        let parse = FractionParser::parse(Rule::program, "a,b,c");
        println!("{:#?}", parse);
        let parse = FractionParser::parse(Rule::program, "a = 2");
        println!("{:#?}", parse);
        assert!(false);
    }
}