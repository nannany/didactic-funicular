use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

mod disk;
mod buffer;

fn main() {
    let dialect = GenericDialect {};

    let sql = "SELECT a, b, 123, myfunc(b) \
    FROM table_1 \
    WHERE a > b AND b < 100 \
    ORDER BY a DESC, b";

    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    println!("AST: {:?}", ast);
}
