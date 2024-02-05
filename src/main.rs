use::std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path  = &args[1];
    match read_file(file_path) {
        Ok(contents) => {
            // println!("File contents:\n{}", contents);
            let lmao = tokenize(&contents);
            println!("{:#?}", lmao);
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(0);
        }
    }
}

fn read_file(path: &str) -> Result<String, String> {
    let buffer = match std::fs::read_to_string(path) {
        Ok(contents) => Ok(contents),
        Err(err) => Err(format!("Error reading file {}: {}", path, err)),
    };
    buffer
}

#[derive(Debug)]
enum TokenType {
    BinaryOperator,
    Equals,
    OpenParen,
    CloseParen,
}
#[allow(dead_code)]
#[derive(Debug)]
struct Token {
    t_type: TokenType,
    t_value: char,
}

fn give_token(t_type: TokenType, t_value: char) -> Token {
    Token {
        t_type,
        t_value,
    }
}

fn tokenize(source_code: &str) -> std::vec::Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let src = source_code;
    let mut buff_arr_src: Vec<char> = src.chars().collect();
    // println!("{:?} " , buff_arr_src);
    while !buff_arr_src.is_empty() {
        let c = buff_arr_src[0];
        match c {
            '(' => tokens.push(give_token(TokenType::OpenParen, c)),
            ')' => tokens.push(give_token(TokenType::CloseParen, c)),
            '+' | '-' | '*' | '/' => tokens.push(give_token(TokenType::BinaryOperator, c)),
            '=' => tokens.push(give_token(TokenType::Equals, c)),
            _ => {} // Ignore other characters
        }
        buff_arr_src.remove(0);
    }
    // println!("lmao{:?}" , buff_arr_src);

    tokens
}