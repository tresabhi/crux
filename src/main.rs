use std::str::FromStr;

use clap::Parser;
use tokens::Token;
mod tokens;

#[derive(Parser)]
struct Cli {
  pattern: String,
  path: std::path::PathBuf,
}

fn tokenize(source: &str) -> Vec<Token> {
  let mut tokens: Vec<Token> = Vec::new();
  let mut expected_token: Option<Token> = None;
  let mut word = String::new();

  for character in source.chars() {
    if expected_token.is_none() {
      if character.is_whitespace() {
        if (word.len() > 0) {
          let token = Token::from_str(&word);

          tokens.push(token.unwrap());

          word = String::new();
        }

        continue;
      } else {
        word.push(character);
      }
    }
  }

  tokens
}

fn main() {
  let args = Cli::parse();
  let content = std::fs::read_to_string(&args.path).expect("could not read file");
  let tokens = tokenize(&content);

  for token in tokens {
    println!("{:?}", token);
  }
}
