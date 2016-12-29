/* parser.rs
 *
 * Takes a Vec of Tokens and returns an AST
 *
 */


/* exp := ( (exp|IDENT) (exp|Number|Identifier)*
 */
use lexer::Token;
use std::iter::Peekable;

/* TODO: Add lambdas */
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum AstNode {
    Expression(Vec<Box<AstNode>>),
    Number(f64),
    String(String),
    Identifier(String),
    Define(String, Box<AstNode>),
    Lambda(Vec<Box<AstNode>>, Box<AstNode>), // contains (list of args, AST of function body)
}

pub fn parse_define<I>(tokens: &mut Peekable<I>) -> AstNode
    where I: Iterator<Item=Token>
{
    let identifier: String;
    let value: Box<AstNode>;

    if let Some(token) = (*tokens).next() {
        if let Token::Identifier(ident) = token {
           identifier = ident;
        }
        else {
            panic!("Define arg 1 expected to be an identifier: {:?}", token);
        }
    }
    else {
        panic!("Unexpected end of token stream");
    }
    value = Box::new(parse(tokens));

    return AstNode::Define(identifier, value);
}

pub fn parse_exp<I>(tokens: &mut Peekable<I>) -> AstNode
    where I: Iterator<Item=Token>
{
    let mut expr: Vec<Box<AstNode>> = Vec::new();
    loop {
        if let Some(&Token::CloseParen) = (*tokens).peek() {
            break;
        }
        expr.push(Box::new(parse(tokens)));
    }
    // Consume CloseParen
    (*tokens).next();

    return AstNode::Expression(expr);
}

pub fn parse<I>(tokens: &mut Peekable<I>) -> AstNode
    where I: Iterator<Item=Token>
{
    if let Some(token) = (*tokens).next() {
        match token {
            Token::OpenParen => match *((*tokens).peek().unwrap()) {
                Token::Define => {
                    (*tokens).next();
                    return parse_define(tokens);
                },
                _ => parse_exp(tokens)
            },
            Token::Number(x) => AstNode::Number(x),
            Token::String(x) => AstNode::String(x),
            Token::Identifier(x) => AstNode::Identifier(x),
            Token::Define => panic!("Unexpected define!"),
            Token::CloseParen => panic!("Unexpected )!"),
        }
    }
    else {
        panic!("Unexpected end of token stream");
    }
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
        let expected_ast = AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("+"))),
                                                 Box::new(AstNode::Number(3.0)),
                                                 Box::new(AstNode::Number(4.0))]);
        let ast = parse(&mut tokens.into_iter().peekable());
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn nested_parse() {
        let tokens = vec![Token::OpenParen, Token::Identifier(String::from("+")),
                          Token::Number(3.0), Token::OpenParen, Token::Identifier(String::from("*")),
                          Token::Number(3.0), Token::Number(4.0), Token::CloseParen, Token::CloseParen];
        let expected_ast = AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("+"))),
                                                 Box::new(AstNode::Number(3.0)),
                                                 Box::new(AstNode::Expression(vec![
                                                    Box::new(AstNode::Identifier(String::from("*"))),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Number(4.0))]))]);
        let ast = parse(&mut tokens.into_iter().peekable());
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn first_arg_nested_parse() {
        let tokens = vec![Token::OpenParen, Token::Identifier(String::from("+")),
                          Token::OpenParen, Token::Identifier(String::from("*")),
                          Token::Number(3.0), Token::Number(4.0), Token::CloseParen,
                          Token::Number(6.0), Token::CloseParen];
        let expected_ast = AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("+"))),
                                                 Box::new(AstNode::Expression(vec![
                                                    Box::new(AstNode::Identifier(String::from("*"))),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Number(4.0))])),
                                                    Box::new(AstNode::Number(6.0))]);
        let ast = parse(&mut tokens.into_iter().peekable());
        assert_eq!(ast, expected_ast);
    }
    // TODO: Add failure cases and string tests
    #[test]
    fn string_parse() {
        let tokens = vec![Token::OpenParen, Token::Identifier(String::from("+")),
                          Token::String(String::from("cat")), Token::String(String::from("wow")), Token::CloseParen];
        let expected_ast = AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("+"))),
                                                 Box::new(AstNode::String(String::from("cat"))),
                                                 Box::new(AstNode::String(String::from("wow")))]);
        let ast = parse(&mut tokens.into_iter().peekable());
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn define_parse() {
        let tokens = vec![Token::OpenParen, Token::Define,
                          Token::Identifier(String::from("LENGTH")), Token::Number(10.0), Token::CloseParen];
        let expected_ast = AstNode::Define(String::from("LENGTH"),
                                                 Box::new(AstNode::Number(10.0)));
        let ast = parse(&mut tokens.into_iter().peekable());
        assert_eq!(ast, expected_ast);
    }

    #[test]
    #[should_panic]
    fn mismatched_paren() {
        let tokens = vec![Token::CloseParen, Token::Define,
                          Token::Identifier(String::from("LENGTH")), Token::Number(10.0), Token::CloseParen];
        parse(&mut tokens.into_iter().peekable());
    }

}
