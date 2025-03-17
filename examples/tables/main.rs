use sqlparser::{ast::Statement, dialect::PostgreSqlDialect, parser::Parser};
use std::fs;

struct Column {
    name: String,
    data_type: String,
    is_nullable: bool,
}

struct Table {
    name: String,
    columns: Vec<Column>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sql = fs::read_to_string("examples/tables/main.sql").expect("Unable to read file");
    let ast = Parser::parse_sql(&PostgreSqlDialect {}, &sql)?;
    for stmt in ast {
        match stmt {
            Statement::CreateTable(create_table) => {
                let name = create_table
                    .name
                    .0
                    .iter()
                    .map(|part| part.to_string())
                    .collect::<Vec<String>>()
                    .join(".");
                println!("{}", name);
                println!("{:#?}", create_table.if_not_exists); // Should emit warning as ideally everything is known
            }
            _ => {
                println!("{:#?}", stmt);
            }
        }
    }
    return Ok(());
}
