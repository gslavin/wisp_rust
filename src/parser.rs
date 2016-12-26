/* parser.rs
 *
 * Takes a Vec of Tokens and returns an AST
 *
 */


/* exp := ( (exp|IDENT) (exp|Number|Identifier)*
 */
use lexer::Token;
use std;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub struct F64Key(u64);

impl F64Key {
    pub fn new(mut val: f64) -> F64Key {
        if val.is_nan() { val = std::f64::NAN } // make all NaNs have the same representation
        unsafe { F64Key(std::mem::transmute(val)) }
    }
    pub fn get(self) -> f64 {
        unsafe { std::mem::transmute(self) }
    }

    pub fn set(&mut self, mut val : f64) {
        if val.is_nan() { val = std::f64::NAN } // make all NaNs have the same representation
        unsafe { *self = std::mem::transmute(val) }
    }
}

/* TODO: Add lambdas */
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum AstNode {
    Expression(Vec<Box<AstNode>>),
    Number(F64Key),
    String(String),
    Identifier(String),
    Define,
    Lambda(Vec<Box<AstNode>>, Box<AstNode>), // contains (list of args, AST of function body)
}

/* TODO: Rust's float's aren't fully ordered so you can't use them with maps (WHYYYYY!!!!)
   Try using the crate ordered_float::OrderedFloat to add ordering??
*/


pub fn parse<I>(tokens: &mut I) -> AstNode
    where I: Iterator<Item=Token>
{
    let mut expr: Vec<Box<AstNode>> = Vec::new();
    while let Some(token) = (*tokens).next() {
        match token {
            Token::OpenParen => expr.push(Box::new(parse(tokens))),
            Token::Number(x) => expr.push(Box::new(AstNode::Number(F64Key::new(x)))),
            Token::String(x) => expr.push(Box::new(AstNode::String(x))),
            Token::Identifier(x) => expr.push(Box::new(AstNode::Identifier(x))),
            Token::Define => expr.push(Box::new(AstNode::Define)),
            Token::CloseParen => break,
        }
    }

    return AstNode::Expression(expr);
}

#[cfg(test)]
mod test {
    use parser::parse;
    use parser::AstNode;
    use parser::F64Key;
    use lexer::Token;

    #[test]
    fn simple_parse() {
        let tokens = vec![Token::OpenParen, Token::Identifier(String::from("+")),
                          Token::Number(3.0), Token::Number(4.0), Token::CloseParen];
        let expected_ast = AstNode::Expression(vec![Box::new(
                           AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("+"))),
                                                 Box::new(AstNode::Number(F64Key::new(3.0))),
                                                 Box::new(AstNode::Number(F64Key::new(4.0)))]))]);
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
                                                 Box::new(AstNode::Number(F64Key::new(3.0))),
                                                 Box::new(AstNode::Expression(vec![
                                                    Box::new(AstNode::Identifier(String::from("*"))),
                                                    Box::new(AstNode::Number(F64Key::new(3.0))),
                                                    Box::new(AstNode::Number(F64Key::new(4.0)))]))]))]);
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
                                                    Box::new(AstNode::Number(F64Key::new(3.0))),
                                                    Box::new(AstNode::Number(F64Key::new(4.0)))])),
                                                    Box::new(AstNode::Number(F64Key::new(3.0)))]))]);
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

    #[test]
    fn define_parse() {
        let tokens = vec![Token::OpenParen, Token::Define,
                          Token::Identifier(String::from("LENGTH")), Token::Number(10.0), Token::CloseParen];
        let expected_ast = AstNode::Expression(vec![Box::new(
                           AstNode::Expression(vec![Box::new(AstNode::Define),
                                                 Box::new(AstNode::Identifier(String::from("LENGTH"))),
                                                 Box::new(AstNode::Number(F64Key::new(10.0)))]))]);
        let ast = parse(&mut tokens.into_iter());
        assert_eq!(ast, expected_ast);
    }

}
