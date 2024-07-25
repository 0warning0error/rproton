use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/sql.pest"]
pub(crate) struct SQLParser;

#[test]
fn parse_something(){
	let result = SQLParser::parse(Rule::sql,"SelecT * froM test_2;");
	assert!(result.is_ok());
}

#[test]
fn parse_create(){
	let result = SQLParser::parse(Rule::sql,"CreAte RanDOM StreAm simida( a1 string);");
	assert!(result.is_ok());
}