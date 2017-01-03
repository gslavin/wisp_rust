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
    Expression(Vec<Box<AstNode>>), // Expression(list of arguments)
    Define(String, Box<AstNode>), // Define(name, value)
    Lambda(Vec<String>, Box<AstNode>), // lambda(list of parameter identifiers, expr)
    If(Box<AstNode>, Box<AstNode>, Box<AstNode>), // If (pred, true expr, false expr)
    Bool(bool),
    Number(f64),
    String(String),
    Identifier(String)
}

pub fn parse_lambda<I>(tokens: &mut Peekable<I>) -> AstNode
    where I: Iterator<Item=Token>
{
    let mut args: Vec<String> = Vec::new();
    let expr: Box<AstNode>;

    if let Some(Token::OpenParen) = (*tokens).next() {
        while let Some(token) = (*tokens).next() {
            match token  {
                Token::CloseParen => break,
                Token::Identifier(ident) => args.push(ident.clone()),
                other => panic!("Lambda arguments must be identifiers: {:?}", other)
            }
        }
    }
    else {
        panic!("Invalid syntax for lambda");
    }
    expr = Box::new(parse(tokens));

    // Consume CloseParen
    match (*tokens).next().unwrap() {
        Token::CloseParen => AstNode::Lambda(args, expr),
        _ => panic!("too many arguments for lambda! Expected CloseParen")
    }
}

pub fn parse_if<I>(tokens: &mut Peekable<I>) -> AstNode
    where I: Iterator<Item=Token>
{
    let pred: Box<AstNode>;
    let if_path: Box<AstNode>;
    let else_path: Box<AstNode>;

    pred = Box::new(parse(tokens));
    if_path = Box::new(parse(tokens));
    else_path = Box::new(parse(tokens));
    // Consume CloseParen
    match (*tokens).next().unwrap() {
        Token::CloseParen => {},
        token => panic!("Expected close paren in if statement but found: {:?}", token)
    }

    return AstNode::If(pred, if_path, else_path);
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
    // Consume CloseParen
    (*tokens).next();

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
                Token::Lambda => {
                    (*tokens).next();
                    return parse_lambda(tokens);
                },
                Token::If => {
                    (*tokens).next();
                    return parse_if(tokens);
                },
                _ => parse_exp(tokens)
            },
            Token::Bool(x) => AstNode::Bool(x),
            Token::Number(x) => AstNode::Number(x),
            Token::String(x) => AstNode::String(x),
            Token::Identifier(x) => AstNode::Identifier(x),
            Token::Define => panic!("Unexpected define!"),
            Token::Lambda => panic!("Unexpected lambda!"),
            Token::If => panic!("Unexpected if!"),
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
    fn define_lambda_parse() {
        //(define my_func (lambda (x y z) (* x y z)))
        let tokens = vec![Token::OpenParen, Token::Define, Token::Identifier(String::from("my_func")), 
            Token::OpenParen, Token::Lambda, Token::OpenParen, Token::Identifier(String::from("x")), Token::CloseParen,
            Token::OpenParen, Token::Identifier(String::from("*")), Token::Identifier(String::from("x")),
            Token::Identifier(String::from("x")), Token::CloseParen, Token::CloseParen];

        let expected_ast = AstNode::Define(String::from("my_func"), Box::new(AstNode::Lambda(vec![String::from("x")],
                                           Box::new(AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("*"))),
                                                 Box::new(AstNode::Identifier(String::from("x"))),
                                                 Box::new(AstNode::Identifier(String::from("x")))])))));
        let ast = parse(&mut tokens.into_iter().peekable());
        assert_eq!(ast, expected_ast);
    }

    #[test]
    #[should_panic]
    fn mismatched_paren_beginning() {
        let tokens = vec![Token::CloseParen, Token::Define,
                          Token::Identifier(String::from("LENGTH")), Token::Number(10.0), Token::CloseParen];
        parse(&mut tokens.into_iter().peekable());
    }

    #[test]
    #[should_panic]
    fn mismatched_paren_end() {
        let tokens = vec![Token::Define,
                          Token::Identifier(String::from("LENGTH")), Token::Number(10.0), Token::CloseParen, Token::CloseParen];
        parse(&mut tokens.into_iter().peekable());
    }

    #[test]
    fn lambda_parse() {
        // (lambda (x) (* x x))
        let tokens = vec![Token::OpenParen, Token::Lambda,
            Token::OpenParen, Token::Identifier(String::from("x")), Token::CloseParen,
            Token::OpenParen, Token::Identifier(String::from("*")), Token::Identifier(String::from("x")),
            Token::Identifier(String::from("x")), Token::CloseParen, Token::CloseParen];

        let expected_ast = AstNode::Lambda(vec![String::from("x")],
                                           Box::new(AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("*"))),
                                                 Box::new(AstNode::Identifier(String::from("x"))),
                                                 Box::new(AstNode::Identifier(String::from("x")))])));
        let ast = parse(&mut tokens.into_iter().peekable());
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn lambda_exp_parse() {
        // ((lambda (x) (* x x)) 4)
        let tokens = vec![Token::OpenParen, Token::OpenParen, Token::Lambda,
            Token::OpenParen, Token::Identifier(String::from("x")), Token::CloseParen,
            Token::OpenParen, Token::Identifier(String::from("*")), Token::Identifier(String::from("x")),
            Token::Identifier(String::from("x")), Token::CloseParen, Token::CloseParen, Token::Number(4.0), Token::CloseParen];

        let expected_ast = AstNode::Expression(vec![
                            Box::new(AstNode::Lambda(vec![String::from("x")],
                                        Box::new(AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("*"))),
                                            Box::new(AstNode::Identifier(String::from("x"))),
                                            Box::new(AstNode::Identifier(String::from("x")))])))),
                            Box::new(AstNode::Number(4.0))]);
        let ast = parse(&mut tokens.into_iter().peekable());
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn lambda_parse_const_expr() {
        // (lambda (x) 1)
        let tokens = vec![Token::OpenParen, Token::Lambda, Token::OpenParen, Token::Identifier(String::from("x")), Token::CloseParen,
            Token::Number(1.0), Token::CloseParen];

        let expected_ast = AstNode::Lambda(vec![String::from("x")],
                                           Box::new(AstNode::Number(1.0)));
        let ast = parse(&mut tokens.into_iter().peekable());
        assert_eq!(ast, expected_ast);
    }

    #[test]
    #[should_panic]
    fn lambda_parse_malformed_args() {
        let tokens = vec![Token::OpenParen, Token::Lambda,
            Token::OpenParen, Token::Number(10.0), Token::CloseParen,
            Token::OpenParen, Token::Identifier(String::from("*")), Token::Identifier(String::from("x")),
            Token::Identifier(String::from("x")), Token::CloseParen, Token::CloseParen];

        parse(&mut tokens.into_iter().peekable());
    }

    #[test]
    #[should_panic]
    fn lambda_unexpected() {
        let tokens = vec![Token::Lambda,
            Token::OpenParen, Token::Number(10.0), Token::CloseParen,
            Token::OpenParen, Token::Identifier(String::from("*")), Token::Identifier(String::from("x")),
            Token::Identifier(String::from("x")), Token::CloseParen, Token::CloseParen];

        parse(&mut tokens.into_iter().peekable());
    }

    #[test]
    fn if_test() {
        // (if true false true)
        let tokens = vec![Token::OpenParen, Token::If,
            Token::Bool(true), Token::Bool(false), Token::Bool(true), Token::CloseParen];
        let expected_ast =  AstNode::If(Box::new(AstNode::Bool(true)), Box::new(AstNode::Bool(false)),
            Box::new(AstNode::Bool(true)));

        let ast = parse(&mut tokens.into_iter().peekable());
        assert_eq!(ast, expected_ast);
    }

    #[test]
    #[should_panic]
    fn if_test_too_many_args() {
        // (if true false true true)
        let tokens = vec![Token::OpenParen, Token::If,
            Token::Bool(true), Token::Bool(false), Token::Bool(true), Token::Bool(true), Token::CloseParen];

        parse(&mut tokens.into_iter().peekable());
    }

    #[test]
    #[should_panic]
    fn if_expected_test() {
        // if true false true true)
        let tokens = vec![Token::If,
            Token::Bool(true), Token::Bool(false), Token::Bool(true), Token::Bool(true), Token::CloseParen];

        parse(&mut tokens.into_iter().peekable());
    }

}
