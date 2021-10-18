use anyhow::Result;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::fs;

fn main() -> Result<()> {
    let sql = fs::read_to_string("sql.txt")?;

    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...
    let ast = Parser::parse_sql(&dialect, sql.as_str())?;

    println!("AST: {:?}", ast[0]);
    Ok(())
}
