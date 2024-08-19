use std::str::FromStr;

#[derive(Debug)]
pub enum Token {
  Keyword(Keyword),
  Inequality(Inequality),
  String(String),
  Variable(String),
}

impl FromStr for Token {
  type Err = ();

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    if let Ok(keyword) = Keyword::from_str(string) {
      return Ok(Token::Keyword(keyword));
    } else if let Ok(inequality) = Inequality::from_str(string) {
      return Ok(Token::Inequality(inequality));
    } else if string.starts_with('"') && string.ends_with('"') {
      return Ok(Token::String(string[1..string.len() - 1].to_string()));
    } else {
      return Ok(Token::Variable(string.to_string()));
    }
  }
}

#[derive(Debug)]
pub enum Inequality {
  Equal,
  NotEqual,
  GreaterThan,
  GreaterThanOrEqual,
  LessThan,
  LessThanOrEqual,
}

impl FromStr for Inequality {
  type Err = ();

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    match string {
      "==" => Ok(Inequality::Equal),
      "!=" => Ok(Inequality::NotEqual),
      ">" => Ok(Inequality::GreaterThan),
      ">=" => Ok(Inequality::GreaterThanOrEqual),
      "<" => Ok(Inequality::LessThan),
      "<=" => Ok(Inequality::LessThanOrEqual),
      _ => Err(()),
    }
  }
}

#[derive(Debug)]
pub enum Keyword {
  Import,
  From,
}

impl FromStr for Keyword {
  type Err = ();

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    match string {
      "import" => Ok(Keyword::Import),
      "from" => Ok(Keyword::From),
      _ => Err(()),
    }
  }
}
