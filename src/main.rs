use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;

mod disk;
mod buffer;

fn main() {
    let dialect = MySqlDialect {};

    let sql = "create table user (id int, name varchar(10));";

    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    println!("AST: {:?}", ast);
}
