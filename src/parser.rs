/* parser.rs
 *
 * Takes a Vec of Tokens and returns an AST
 *
 */


/* exp := ( (exp|IDENT) (exp|Number|Identifier)*
 */
use lexer::Token;
use std;
use std::rc::Rc;

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
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum AstNode {
    Expression(Vec<Rc<AstNode>>),
    Number(F64Key),
    String(String),
    Identifier(String),
    Define,
    Lambda(Vec<Rc<AstNode>>, Rc<AstNode>), // contains (list of args, AST of function body)
}

/* TODO: Rust's float's aren't fully ordered so you can't use them with maps (WHYYYYY!!!!)
   Try using the crate ordered_float::OrderedFloat to add ordering??
*/


pub fn parse<I>(tokens: &mut I) -> AstNode
    where I: Iterator<Item=Token>
{
    let mut expr: Vec<Rc<AstNode>> = Vec::new();
    while let Some(token) = (*tokens).next() {
        match token {
            Token::OpenParen => expr.push(Rc::new(parse(tokens))),
            Token::Number(x) => expr.push(Rc::new(AstNode::Number(F64Key::new(x)))),
            Token::String(x) => expr.push(Rc::new(AstNode::String(x))),
            Token::Identifier(x) => expr.push(Rc::new(AstNode::Identifier(x))),
            Token::Define => expr.push(Rc::new(AstNode::Define)),
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
	use std::rc::Rc;

    #[test]
    fn simple_parse() {
        let tokens = vec![Token::OpenParen, Token::Identifier(String::from("+")),
                          Token::Number(3.0), Token::Number(4.0), Token::CloseParen];
        let expected_ast = AstNode::Expression(vec![Rc::new(
                           AstNode::Expression(vec![Rc::new(AstNode::Identifier(String::from("+"))),
                                                 Rc::new(AstNode::Number(F64Key::new(3.0))),
                                                 Rc::new(AstNode::Number(F64Key::new(4.0)))]))]);
        let ast = parse(&mut tokens.into_iter());
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn nested_parse() {
        let tokens = vec![Token::OpenParen, Token::Identifier(String::from("+")),
                          Token::Number(3.0), Token::OpenParen, Token::Identifier(String::from("*")),
                          Token::Number(3.0), Token::Number(4.0), Token::CloseParen, Token::CloseParen];
        let expected_ast = AstNode::Expression(vec![Rc::new(
                           AstNode::Expression(vec![Rc::new(AstNode::Identifier(String::from("+"))),
                                                 Rc::new(AstNode::Number(F64Key::new(3.0))),
                                                 Rc::new(AstNode::Expression(vec![
                                                    Rc::new(AstNode::Identifier(String::from("*"))),
                                                    Rc::new(AstNode::Number(F64Key::new(3.0))),
                                                    Rc::new(AstNode::Number(F64Key::new(4.0)))]))]))]);
        let ast = parse(&mut tokens.into_iter());
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn first_arg_nested_parse() {
        let tokens = vec![Token::OpenParen, Token::Identifier(String::from("+")),
                          Token::OpenParen, Token::Identifier(String::from("*")),
                          Token::Number(3.0), Token::Number(4.0), Token::CloseParen,
                          Token::Number(3.0), Token::CloseParen];
        let expected_ast = AstNode::Expression(vec![Rc::new(
                           AstNode::Expression(vec![Rc::new(AstNode::Identifier(String::from("+"))),
                                                 Rc::new(AstNode::Expression(vec![
                                                    Rc::new(AstNode::Identifier(String::from("*"))),
                                                    Rc::new(AstNode::Number(F64Key::new(3.0))),
                                                    Rc::new(AstNode::Number(F64Key::new(4.0)))])),
                                                    Rc::new(AstNode::Number(F64Key::new(3.0)))]))]);
        let ast = parse(&mut tokens.into_iter());
        assert_eq!(ast, expected_ast);
    }
    // TODO: Add failure cases and string tests
    #[test]
    fn string_parse() {
        let tokens = vec![Token::OpenParen, Token::Identifier(String::from("+")),
                          Token::String(String::from("cat")), Token::String(String::from("wow")), Token::CloseParen];
        let expected_ast = AstNode::Expression(vec![Rc::new(
                           AstNode::Expression(vec![Rc::new(AstNode::Identifier(String::from("+"))),
                                                 Rc::new(AstNode::String(String::from("cat"))),
                                                 Rc::new(AstNode::String(String::from("wow")))]))]);
        let ast = parse(&mut tokens.into_iter());
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn define_parse() {
        let tokens = vec![Token::OpenParen, Token::Define,
                          Token::Identifier(String::from("LENGTH")), Token::Number(10.0), Token::CloseParen];
        let expected_ast = AstNode::Expression(vec![Rc::new(
                           AstNode::Expression(vec![Rc::new(AstNode::Define),
                                                 Rc::new(AstNode::Identifier(String::from("LENGTH"))),
                                                 Rc::new(AstNode::Number(F64Key::new(10.0)))]))]);
        let ast = parse(&mut tokens.into_iter());
        assert_eq!(ast, expected_ast);
    }

}
