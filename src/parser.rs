/* parser.rs
 *
 * Takes a Vec of Tokens and returns an AST
 *
 */


/* exp := ( (exp|IDENT) (exp|Number|Identifier)*
 */
use lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Node {
    Expression(Vec<Box<Node>>),
    Number(f32),
    Identifier(String),
}



pub fn parse<I>(tokens: &mut I) -> Node
    where I: Iterator<Item=Token>
{
    let mut expr: Vec<Box<Node>> = Vec::new();
    while let Some(token) = (*tokens).next() {
        match token {
            Token::OpenParen => expr.push(Box::new(parse(tokens))),
            Token::Number(x) => expr.push(Box::new(Node::Number(x))),
            Token::Identifier(x) => expr.push(Box::new(Node::Identifier(x))),
            Token::CloseParen => break,
        }
    }

    return Node::Expression(expr);
}

#[cfg(test)]
mod test {
    use parser::parse;
    use parser::Node;
    use lexer::Token;

    #[test]
    fn simple_parse() {
        let tokens = vec![Token::OpenParen, Token::Identifier(String::from("+")),
                          Token::Number(3.0), Token::Number(4.0), Token::CloseParen];
        let expected_ast = Node::Expression(vec![Box::new(
                           Node::Expression(vec![Box::new(Node::Identifier(String::from("+"))),
                                                 Box::new(Node::Number(3.0)), 
                                                 Box::new(Node::Number(4.0))]))]);
        let ast = parse(&mut tokens.into_iter());
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn nested_parse() {
        let tokens = vec![Token::OpenParen, Token::Identifier(String::from("+")),
                          Token::Number(3.0), Token::OpenParen, Token::Identifier(String::from("*")),
                          Token::Number(3.0), Token::Number(4.0), Token::CloseParen, Token::CloseParen];
        let expected_ast = Node::Expression(vec![Box::new(
                           Node::Expression(vec![Box::new(Node::Identifier(String::from("+"))),
                                                 Box::new(Node::Number(3.0)), 
                                                 Box::new(Node::Expression(vec![
                                                    Box::new(Node::Identifier(String::from("*"))),
                                                    Box::new(Node::Number(3.0)), 
                                                    Box::new(Node::Number(4.0))]))]))]);
        let ast = parse(&mut tokens.into_iter());
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn first_arg_nested_parse() {
        let tokens = vec![Token::OpenParen, Token::Identifier(String::from("+")),
                          Token::OpenParen, Token::Identifier(String::from("*")),
                          Token::Number(3.0), Token::Number(4.0), Token::CloseParen,
                          Token::Number(3.0), Token::CloseParen];
        let expected_ast = Node::Expression(vec![Box::new(
                           Node::Expression(vec![Box::new(Node::Identifier(String::from("+"))),
                                                 Box::new(Node::Expression(vec![
                                                    Box::new(Node::Identifier(String::from("*"))),
                                                    Box::new(Node::Number(3.0)),
                                                    Box::new(Node::Number(4.0))])),
                                                    Box::new(Node::Number(3.0))]))]);
        let ast = parse(&mut tokens.into_iter());
        assert_eq!(ast, expected_ast);
    }
    // TODO: Add failure cases
}
