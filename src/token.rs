use std::str::FromStr;

#[derive(Debug)]
pub struct Token {
	token: String,
	tokenType: TokenType,
}

#[derive(Debug)]
enum TokenType{
	SELECT,
	ALL,
	FROM,
	IDENTIFIER,
}

impl Token{

	pub fn tokenize(query: String) -> Vec<Token> {
		let mut tokens = Vec::new();
		for str_token in query.split(" ") {
			let token = Token::makeToken(str_token);
			tokens.push(token);
		}
		tokens
	}

	fn makeToken(str_token: &str) -> Token {
		let token_type = match str_token {
			"SELECT" => TokenType::SELECT,
			"FROM" => TokenType::FROM,
			"*" => TokenType::ALL,
			_ => TokenType::IDENTIFIER,
		};
		Token {token: str_token.to_string(), tokenType: token_type}
	}
}