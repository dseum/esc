use std::collections::HashMap;

use sqlparser::{
    ast::{Query, Select, SetExpr, Statement},
    dialect::PostgreSqlDialect,
    parser::Parser,
};

struct Aliases {
    aliases: HashMap<String, String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sql = "SELECT u.id FROM users AS u WHERE u.id = 1 LIMIT 1";
    let ast = Parser::parse_sql(&PostgreSqlDialect {}, sql)?;
    for item in &ast {
        match item {
            Statement::Query(query) => match &*query.body {
                SetExpr::Select(select) => {
                    for item in &select.projection {
                        match item {
                            sqlparser::ast::SelectItem::UnnamedExpr(expr) => {
                                println!("{:#?}", expr);
                            }
                            sqlparser::ast::SelectItem::ExprWithAlias { expr, alias } => {
                                println!("{:#?}", expr);
                                println!("{:#?}", alias);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
            _ => {
                println!("{:#?}", item);
            }
        }
    }
    println!("{:#?}", ast);
    return Ok(());
}
