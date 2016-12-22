/* parser.rs
 *
 * Takes a Vec of Tokens and returns an AST
 *
 */


/* exp := ( (exp|IDENT) (exp|Number|Identifier)*
 */
use lexer::Token;

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Expression(Vec<Box<AstNode>>),
    Number(f64),
    String(String),
    Identifier(String),
    Define,
}

pub fn parse<I>(tokens: &mut I) -> AstNode
    where I: Iterator<Item=Token>
{
    let mut expr: Vec<Box<AstNode>> = Vec::new();
    while let Some(token) = (*tokens).next() {
        match token {
            Token::OpenParen => expr.push(Box::new(parse(tokens))),
            Token::Number(x) => expr.push(Box::new(AstNode::Number(x))),
            Token::String(x) => expr.push(Box::new(AstNode::String(x))),
            Token::Identifier(x) => expr.push(Box::new(AstNode::Identifier(x))),
            Token::CloseParen => break,
        }
    }

    return AstNode::Expression(expr);
}

#[cfg(test)]
mod test {
    use parser::parse;
    use parser::AstNode;
    use lexer::Token;

    #[test]
    fn simple_parse() {
        let tokens = vec![Token::OpenParen, Token::Identifier(String::from("+")),
                          Token::Number(3.0), Token::Number(4.0), Token::CloseParen];
        let expected_ast = AstNode::Expression(vec![Box::new(
                           AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("+"))),
                                                 Box::new(AstNode::Number(3.0)),
                                                 Box::new(AstNode::Number(4.0))]))]);
        let ast = parse(&mut tokens.into_iter());
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn nested_parse() {
        let tokens = vec![Token::OpenParen, Token::Identifier(String::from("+")),
                          Token::Number(3.0), Token::OpenParen, Token::Identifier(String::from("*")),
                          Token::Number(3.0), Token::Number(4.0), Token::CloseParen, Token::CloseParen];
        let expected_ast = AstNode::Expression(vec![Box::new(
                           AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("+"))),
                                                 Box::new(AstNode::Number(3.0)),
                                                 Box::new(AstNode::Expression(vec![
                                                    Box::new(AstNode::Identifier(String::from("*"))),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Number(4.0))]))]))]);
        let ast = parse(&mut tokens.into_iter());
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn first_arg_nested_parse() {
        let tokens = vec![Token::OpenParen, Token::Identifier(String::from("+")),
                          Token::OpenParen, Token::Identifier(String::from("*")),
                          Token::Number(3.0), Token::Number(4.0), Token::CloseParen,
                          Token::Number(3.0), Token::CloseParen];
        let expected_ast = AstNode::Expression(vec![Box::new(
                           AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("+"))),
                                                 Box::new(AstNode::Expression(vec![
                                                    Box::new(AstNode::Identifier(String::from("*"))),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Number(4.0))])),
                                                    Box::new(AstNode::Number(3.0))]))]);
        let ast = parse(&mut tokens.into_iter());
        assert_eq!(ast, expected_ast);
    }
    // TODO: Add failure cases and string tests
    #[test]
    fn string_parse() {
        let tokens = vec![Token::OpenParen, Token::Identifier(String::from("+")),
                          Token::String(String::from("cat")), Token::String(String::from("wow")), Token::CloseParen];
        let expected_ast = AstNode::Expression(vec![Box::new(
                           AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("+"))),
                                                 Box::new(AstNode::String(String::from("cat"))),
                                                 Box::new(AstNode::String(String::from("wow")))]))]);
        let ast = parse(&mut tokens.into_iter());
        assert_eq!(ast, expected_ast);
    }

}
