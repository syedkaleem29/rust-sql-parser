use crate::token::Token;

struct SQLParser;

struct Query {
	qType: QueryType,
	qCols: Column,
	qFrom: Table,
}

struct Column {
	tableName: String,
	columnName: String,
}

struct Table {
	tableName: String
}

enum QueryType{
	SELECT,
	UPDATE,
	DELETE
}

impl SQLParser{
	fn parser(query: String) -> Query {
		let tokens = Token::tokenize(query);
		for token in tokens {
			println!("{:?}", token);
		}
		unimplemented!()
	}
}

#[cfg(test)]
mod test{
	use super::*;

	#[test]
	fn test_parser() {
		let s = "SELECT * FROM Table;".to_string();

		SQLParser::parser(s);
	}
}