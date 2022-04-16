# didactic-funicular
rust で rdbms

# 構造

| 役割 | ソースコード |
| :-- | :-- |
| 構文解析器 | [sqlparser](https://github.com/sqlparser-rs/sqlparser-rs) |
| クエリプランナー | .rs |
| クエリエクスキュータ | query.rs |
| アクセスメソッド | btree.rs,  |
| バッファプールマネージャ | buffer.rs |
| ディスクマネージャ | disk.rs |

## 構文解析器

文字列として受け取ったSQL文を抽象構文木(Abstract Syntax Tree)に変換する機能を持つ。

例えば
```sql
SELECT a from my_table;
```

というSQLがASTに変換されると
```
AST: [Query(Query { with: None, body: Select(Select { distinct: false, top: None, projection: [UnnamedExpr(Identifier(Ident { value: "a", quote_style: None }))], from: [TableWithJoins { relation: Table { name: ObjectName([Ident { value: "my_table", quote_style: None }]), alias: None, args: [], with_hints: [] }, joins: [] }], lateral_views: [], selection: None, group_by: [], cluster_by: [], distribute_by: [], sort_by: [], having: None }), order_by: [], limit: None, offset: None, fetch: None, lock: None })]
```

になる。

```sql
CREATE TABLE user (id INT, name VARCHAR(10));
```

というSQLだと

```
AST: [CreateTable { or_replace: false, temporary: false, external: false, if_not_exists: false, name: ObjectName([Ident { value: "user", quote_style: None }]), columns: [ColumnDef { name: Ident { value: "id", quote_style: None }, data_type: Int(None), collation: None, options: [] }, ColumnDef { name: Ident { value: "name", quote_style: None }, data_type: Varchar(Some(10)), collation: None, options: [] }], constraints: [], hive_distribution: NONE, hive_formats: Some(HiveFormat { row_format: None, storage: None, location: None }), table_properties: [], with_options: [], file_format: None, location: None, query: None, without_rowid: false, like: None, engine: None, default_charset: None, collation: None }]
```

## クエリプランナー

オプティマイザーとも呼ばれている機能。
ASTを元に実行計画を作成する。

# origin

https://github.com/KOBA789/relly/blob/wdb/LICENSE
