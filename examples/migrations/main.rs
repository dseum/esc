use sqlparser::{dialect::PostgreSqlDialect, parser::Parser};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

#[derive(Debug)]
struct PostgresSqlAbstractSchemaTree {
    schemas: HashMap<String, PostgreSqlSchema>,
}

impl PostgresSqlAbstractSchemaTree {
    fn new() -> Self {
        PostgresSqlAbstractSchemaTree {
            schemas: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct PostgreSqlSchema {
    name: String,
    tables: HashMap<String, PostgreSqlTable>,
}

impl PostgreSqlSchema {
    fn new(name: String) -> Self {
        PostgreSqlSchema {
            name,
            tables: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct PostgreSqlTable {
    name: String,
    columns: HashMap<String, String>,
    constraints: HashSet<String>,
}

impl PostgreSqlTable {
    fn new(name: String) -> Self {
        PostgreSqlTable {
            name,
            columns: HashMap::new(),
            constraints: HashSet::new(),
        }
    }
}

fn process(
    migrations_dir: &Path,
) -> Result<PostgresSqlAbstractSchemaTree, Box<dyn std::error::Error>> {
    let mut entries: Vec<_> = fs::read_dir(migrations_dir)?
        .filter_map(Result::ok)
        .collect();
    entries.sort_by_key(|entry| entry.file_name().to_string_lossy().into_owned());
    let mut ast = PostgresSqlAbstractSchemaTree::new();
    for entry in entries {
        let entry = entry;
        let file_path = entry.path();
        let stmts = Parser::parse_sql(&PostgreSqlDialect {}, &file_path.to_string_lossy())?;
        for stmt in stmts {
            match stmt {
                sqlparser::ast::Statement::CreateTable(create_table) => {
                    let mut schema_name: String = "public".to_string();
                    let table_name: String;
                    let name_len = create_table.name.0.len();
                    if name_len == 1 {
                        table_name = create_table.name.0[0].to_string();
                    } else if name_len == 2 {
                        schema_name = create_table.name.0[0].to_string();
                        table_name = create_table.name.0[1].to_string();
                    } else {
                        return Err("only single schema is supported".into());
                    }
                    if !create_table.if_not_exists
                        && ast.schemas.contains_key(&schema_name)
                        && ast.schemas[&schema_name].tables.contains_key(&table_name)
                    {
                        return Err(format!("Table {} already exists", table_name).into());
                    }
                    let mut columns_map = HashMap::new();
                    for column_def in create_table.columns {
                        let column_name = column_def.name.to_string();
                        let data_type = column_def.data_type.to_string();
                        columns_map.insert(column_name, data_type);
                    }
                    let table = PostgreSqlTable {
                        name: table_name.clone(),
                        columns: columns_map,
                        constraints: HashSet::new(),
                    };
                    ast.schemas
                        .entry(schema_name.clone())
                        .or_insert(PostgreSqlSchema {
                            name: schema_name,
                            tables: HashMap::new(),
                        })
                        .tables
                        .insert(table_name, table);
                }
                sqlparser::ast::Statement::AlterTable {
                    name,
                    if_exists,
                    operations,
                    ..
                } => {
                    let schema_name = "public".to_string();
                    let table_name = name.0[0].to_string();
                    if let Some(schema) = ast.schemas.get_mut(&schema_name) {
                        if let Some(table) = schema.tables.get_mut(&table_name) {
                            for alter_item in operations {
                                match alter_item {
                                    sqlparser::ast::AlterTableOperation::AddColumn {
                                        column_def,
                                        column_keyword,
                                        column_position,
                                        if_not_exists,
                                    } => {
                                        let column_name = column_def.name.to_string();
                                        let data_type = column_def.data_type.to_string();
                                        table.columns.insert(column_name, data_type);
                                    }
                                    sqlparser::ast::AlterTableOperation::DropColumn {
                                        drop_behavior,
                                        column_name,
                                        if_exists,
                                    } => {
                                        table.columns.remove(&column_name.to_string());
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    Ok(ast)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let migrations_dir = Path::new("examples/migrations/migrations");
    let schema = process(migrations_dir)?;
    println!("{:#?}", schema);
    Ok(())
}
