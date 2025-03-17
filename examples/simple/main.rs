use sqlparser::{dialect::PostgreSqlDialect, parser::Parser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sql = "SELECT * FROM users WHERE id = 1 LIMIT 1";
    let ast = Parser::parse_sql(&PostgreSqlDialect {}, sql)?;
    println!("{:#?}", ast);
    return Ok(());
}
