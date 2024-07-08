
use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    match read_file(file_path) {
        Ok(contents) => {
            println!("{:#?}", tokenize(&contents));
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}

fn read_file(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|err| format!("Error reading file {}: {}", path, err))
}

#[derive(Debug, PartialEq, Clone)]
enum TokenType {
    BinaryOperator,
    Equals,
    OpenParen,
    CloseParen,
    Semicolon,
    Alphanumeric,
    Numeric,
    Identifier,
    Let,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Token {
    t_type: TokenType,
    t_value: String,
}

fn give_token(t_type: TokenType, t_value: String) -> Token {
    Token { t_type, t_value }
}

fn tokenize(source_code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buff_arr_src: Vec<char> = source_code.chars().collect();
    while !buff_arr_src.is_empty() {
        let c = buff_arr_src[0];
        match c {
            '(' => tokens.push(give_token(TokenType::OpenParen, c.to_string())),
            ')' => tokens.push(give_token(TokenType::CloseParen, c.to_string())),
            '+' | '-' | '*' | '/' => tokens.push(give_token(TokenType::BinaryOperator, c.to_string())),
            '=' => tokens.push(give_token(TokenType::Equals, c.to_string())),
            ';' => tokens.push(give_token(TokenType::Semicolon, c.to_string())),
            _ => {} // Ignore other characters
        }
        if c.is_digit(10) {
            let mut num = String::new();
            while !buff_arr_src.is_empty() && buff_arr_src[0].is_digit(10) {
                num.push(buff_arr_src.remove(0));
            }
            tokens.push(give_token(TokenType::Numeric, num));
            continue;
        }
        else if c.is_alphabetic() {
            let mut iden = String::new();
            while !buff_arr_src.is_empty() && buff_arr_src[0].is_alphabetic() {
                iden.push(buff_arr_src.remove(0));
            }
            tokens.push(give_token(TokenType::Alphanumeric , iden));
            continue;
        }

        buff_arr_src.remove(0);
    }
    tokens
}
