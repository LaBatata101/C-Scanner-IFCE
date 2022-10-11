use scanner::{KeywordType, OperatorType, Scanner, Token};

#[test]
fn test_tokenize_assign_int() {
    let scanner = Scanner::new("int x = 10;");
    assert_eq!(
        scanner.tokens(),
        vec![
            Token::Keyword(KeywordType::Int),
            Token::Id("x".to_string()),
            Token::Operator(OperatorType::Assign),
            Token::Number("10".to_string()),
            Token::SemiColon,
            Token::Eof,
        ]
    )
}

#[test]
fn test_tokenize_assign_float_with_f_postfix() {
    let scanner = Scanner::new("float x = 10.5f;");
    assert_eq!(
        scanner.tokens(),
        vec![
            Token::Keyword(KeywordType::Float),
            Token::Id("x".to_string()),
            Token::Operator(OperatorType::Assign),
            Token::Number("10.5".to_string()),
            Token::SemiColon,
            Token::Eof,
        ]
    )
}

#[test]
fn test_tokenize_assign_float_without_f_postfix() {
    let scanner = Scanner::new("float x = 10.5;");
    assert_eq!(
        scanner.tokens(),
        vec![
            Token::Keyword(KeywordType::Float),
            Token::Id("x".to_string()),
            Token::Operator(OperatorType::Assign),
            Token::Number("10.5".to_string()),
            Token::SemiColon,
            Token::Eof,
        ]
    )
}

#[test]
fn test_tokenize_assign_double_without_f_postfix() {
    let scanner = Scanner::new("double x = 10.5;");
    assert_eq!(
        scanner.tokens(),
        vec![
            Token::Keyword(KeywordType::Double),
            Token::Id("x".to_string()),
            Token::Operator(OperatorType::Assign),
            Token::Number("10.5".to_string()),
            Token::SemiColon,
            Token::Eof,
        ]
    )
}

#[test]
fn test_tokenize_assign_double_with_f_postfix() {
    let scanner = Scanner::new("double x = 10.5f;");
    assert_eq!(
        scanner.tokens(),
        vec![
            Token::Keyword(KeywordType::Double),
            Token::Id("x".to_string()),
            Token::Operator(OperatorType::Assign),
            Token::Number("10.5".to_string()),
            Token::SemiColon,
            Token::Eof,
        ]
    )
}

#[test]
fn test_tokenize_float_number_whiout_integer_part() {
    let scanner = Scanner::new("float x = .5;");
    assert_eq!(
        scanner.tokens(),
        vec![
            Token::Keyword(KeywordType::Float),
            Token::Id("x".to_string()),
            Token::Operator(OperatorType::Assign),
            Token::Number(".5".to_string()),
            Token::SemiColon,
            Token::Eof,
        ]
    )
}

#[test]
fn test_tokenize_add() {
    let scanner = Scanner::new("int x = 10 + 32;");
    assert_eq!(
        scanner.tokens(),
        vec![
            Token::Keyword(KeywordType::Int),
            Token::Id("x".to_string()),
            Token::Operator(OperatorType::Assign),
            Token::Number("10".to_string()),
            Token::Operator(OperatorType::Plus),
            Token::Number("32".to_string()),
            Token::SemiColon,
            Token::Eof,
        ]
    )
}

#[test]
fn test_tokenize_subtract() {
    let scanner = Scanner::new("int x = 10 - 32;");
    assert_eq!(
        scanner.tokens(),
        vec![
            Token::Keyword(KeywordType::Int),
            Token::Id("x".to_string()),
            Token::Operator(OperatorType::Assign),
            Token::Number("10".to_string()),
            Token::Operator(OperatorType::Minus),
            Token::Number("32".to_string()),
            Token::SemiColon,
            Token::Eof,
        ]
    )
}

#[test]
fn test_tokenize_multiply() {
    let scanner = Scanner::new("int x = 10 * 32;");
    assert_eq!(
        scanner.tokens(),
        vec![
            Token::Keyword(KeywordType::Int),
            Token::Id("x".to_string()),
            Token::Operator(OperatorType::Assign),
            Token::Number("10".to_string()),
            Token::Operator(OperatorType::Asterisk),
            Token::Number("32".to_string()),
            Token::SemiColon,
            Token::Eof,
        ]
    )
}

#[test]
fn test_tokenize_divide() {
    let scanner = Scanner::new("int x = 10 / 32;");
    assert_eq!(
        scanner.tokens(),
        vec![
            Token::Keyword(KeywordType::Int),
            Token::Id("x".to_string()),
            Token::Operator(OperatorType::Assign),
            Token::Number("10".to_string()),
            Token::Operator(OperatorType::Divide),
            Token::Number("32".to_string()),
            Token::SemiColon,
            Token::Eof,
        ]
    )
}

#[test]
fn test_tokenize_add_equal() {
    let scanner = Scanner::new("x += 32;");
    assert_eq!(
        scanner.tokens(),
        vec![
            Token::Id("x".to_string()),
            Token::Operator(OperatorType::PlusEqual),
            Token::Number("32".to_string()),
            Token::SemiColon,
            Token::Eof,
        ]
    )
}

#[test]
fn test_tokenize_subtract_equal() {
    let scanner = Scanner::new("x -= 32;");
    assert_eq!(
        scanner.tokens(),
        vec![
            Token::Id("x".to_string()),
            Token::Operator(OperatorType::MinusEqual),
            Token::Number("32".to_string()),
            Token::SemiColon,
            Token::Eof,
        ]
    )
}

#[test]
fn test_tokenize_divide_equal() {
    let scanner = Scanner::new("x /= 32;");
    assert_eq!(
        scanner.tokens(),
        vec![
            Token::Id("x".to_string()),
            Token::Operator(OperatorType::DivideEqual),
            Token::Number("32".to_string()),
            Token::SemiColon,
            Token::Eof,
        ]
    )
}

#[test]
fn test_tokenize_multiply_equal() {
    let scanner = Scanner::new("x *= 32;");
    assert_eq!(
        scanner.tokens(),
        vec![
            Token::Id("x".to_string()),
            Token::Operator(OperatorType::AsteriskEqual),
            Token::Number("32".to_string()),
            Token::SemiColon,
            Token::Eof,
        ]
    )
}

#[test]
fn test_tokenize_string() {
    let scanner = Scanner::new("char* str = \"Hello World!\";");
    assert_eq!(
        scanner.tokens(),
        vec![
            Token::Keyword(KeywordType::Char),
            Token::Operator(OperatorType::Asterisk),
            Token::Id("str".to_string()),
            Token::Operator(OperatorType::Assign),
            Token::String("Hello World!\0".to_string()),
            Token::SemiColon,
            Token::Eof,
        ]
    )
}

#[test]
fn test_tokenize_char() {
    let scanner = Scanner::new("char c = '0';");
    assert_eq!(
        scanner.tokens(),
        vec![
            Token::Keyword(KeywordType::Char),
            Token::Id("c".to_string()),
            Token::Operator(OperatorType::Assign),
            Token::Char('0'),
            Token::SemiColon,
            Token::Eof,
        ]
    )
}

#[test]
fn test_tokenize_invalid_id() {
    let scanner = Scanner::new("int 12343abc;");
    assert_eq!(
        scanner.tokens(),
        vec![
            Token::Keyword(KeywordType::Int),
            Token::Invalid("Invalid identifier \"12343abc\"!".to_string()),
            Token::SemiColon,
            Token::Eof,
        ]
    )
}

#[test]
fn test_tokenize_ignore_oneline_comments() {
    let src = "
int x = 42;// This is a one line comment!
// This is another one line comment!
printf(\"Hello World!\");
";
    let scanner = Scanner::new(src);
    assert_eq!(
        scanner.tokens(),
        vec![
            Token::Keyword(KeywordType::Int),
            Token::Id("x".to_string()),
            Token::Operator(OperatorType::Assign),
            Token::Number("42".to_string()),
            Token::SemiColon,
            Token::Id("printf".to_string()),
            Token::OpenParen,
            Token::String("Hello World!\0".to_string()),
            Token::CloseParen,
            Token::SemiColon,
            Token::Eof
        ]
    )
}

#[test]
fn test_tokenize_ignore_multiline_comments() {
    let src = "
        /*
        * Multiline
        * Comment!!
        */
int x = 42;
        /*
        * Multiline
        * Comment!!
        */
printf(\"Hello World!\");
";
    let scanner = Scanner::new(src);
    assert_eq!(
        scanner.tokens(),
        vec![
            Token::Keyword(KeywordType::Int),
            Token::Id("x".to_string()),
            Token::Operator(OperatorType::Assign),
            Token::Number("42".to_string()),
            Token::SemiColon,
            Token::Id("printf".to_string()),
            Token::OpenParen,
            Token::String("Hello World!\0".to_string()),
            Token::CloseParen,
            Token::SemiColon,
            Token::Eof
        ]
    )
}

#[test]
fn test_tokenize_ignore_multiline_comments2() {
    let src = "
int x = /* Weird place for a comment */ 42;
";
    let scanner = Scanner::new(src);
    assert_eq!(
        scanner.tokens(),
        vec![
            Token::Keyword(KeywordType::Int),
            Token::Id("x".to_string()),
            Token::Operator(OperatorType::Assign),
            Token::Number("42".to_string()),
            Token::SemiColon,
            Token::Eof
        ]
    )
}

#[test]
fn test_tokenize_ignore_multiline_comments3() {
    let src = "
/*
* Multiple multiline comments!
*/
/*
* Multiple multiline comments!
*/
/*
* Multiple multiline comments!
*/
";
    let scanner = Scanner::new(src);
    assert_eq!(scanner.tokens(), vec![Token::Eof])
}

#[test]
fn test_tokenize_small_program() {
    let text = "
int main() {
    int a = 10;
    int c;
    for (int i = 0; i <= 10; i++) {
        if (i + a <= 15) {
            c = i;
        }
    }
    return 0;
}";
    let scanner = Scanner::new(text);
    assert_eq!(
        scanner.tokens(),
        vec![
            Token::Keyword(KeywordType::Int),
            Token::Id("main".to_string()),
            Token::OpenParen,
            Token::CloseParen,
            Token::OpenBrace,
            Token::Keyword(KeywordType::Int),
            Token::Id("a".to_string()),
            Token::Operator(OperatorType::Assign),
            Token::Number("10".to_string()),
            Token::SemiColon,
            Token::Keyword(KeywordType::Int),
            Token::Id("c".to_string()),
            Token::SemiColon,
            Token::Keyword(KeywordType::For),
            Token::OpenParen,
            Token::Keyword(KeywordType::Int),
            Token::Id("i".to_string()),
            Token::Operator(OperatorType::Assign),
            Token::Number("0".to_string()),
            Token::SemiColon,
            Token::Id("i".to_string()),
            Token::Operator(OperatorType::LessThanOrEqual),
            Token::Number("10".to_string()),
            Token::SemiColon,
            Token::Id("i".to_string()),
            Token::Operator(OperatorType::Increment),
            Token::CloseParen,
            Token::OpenBrace,
            Token::Keyword(KeywordType::If),
            Token::OpenParen,
            Token::Id("i".to_string()),
            Token::Operator(OperatorType::Plus),
            Token::Id("a".to_string()),
            Token::Operator(OperatorType::LessThanOrEqual),
            Token::Number("15".to_string()),
            Token::CloseParen,
            Token::OpenBrace,
            Token::Id("c".to_string()),
            Token::Operator(OperatorType::Assign),
            Token::Id("i".to_string()),
            Token::SemiColon,
            Token::CloseBrace,
            Token::CloseBrace,
            Token::Keyword(KeywordType::Return),
            Token::Number("0".to_string()),
            Token::SemiColon,
            Token::CloseBrace,
            Token::Eof,
        ]
    )
}
