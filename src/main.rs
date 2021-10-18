use std::{fs, io};

use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

fn main() -> io::Result<()> {
    let sql = fs::read_to_string("sql.txt")?;

    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...
    let ast = Parser::parse_sql(&dialect, sql.as_str()).unwrap();

    println!("AST: {:?}", ast[0]);
    Ok(())
}
