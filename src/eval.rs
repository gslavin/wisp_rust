/* eval.rs
 *
 * Takes an AST and returns a result
 *
 */
use parser::AstNode;

fn reduce<F>(args: &[Box<AstNode>], f: F) -> AstNode
    where F: Fn(f64, f64) -> f64
{
    let mut sum: f64;
    match *args[0] {
        AstNode::Number(x) => sum = x,
        _ => panic!("Invalid number arg: {:?}", args[0]),
    }
    for arg in args[1..].iter() {
        match **arg {
            AstNode::Number(x) => sum = f(x, sum),
            _ => panic!("Invalid number arg: {:?}", arg),
        }
    }

    return AstNode::Number(sum);
}

/* Apply the given evaluated arguments to the given operand */
fn apply(op: &AstNode, args: &[Box<AstNode>]) -> Option<AstNode> {
    let ret: Option<AstNode>;

    if let AstNode::Identifier(ref ident) = *op {
        match ident.as_str() {
            "+" => ret = Some(reduce(args, |x, sum| sum + x)),
            "*" => ret = Some(reduce(args, |x, prod| prod * x)),
            // TODO: should variable arg -,/ work?
            "-" => ret = Some(reduce(args, |x, sum| sum - x)),
            "/" => ret = Some(reduce(args, |x, prod| prod / x)),
            _ => panic!("Invalid op: {:?}", ident),
        }
    }
    else {
        if args.len() == 0 {
            match *op {
                AstNode::Number(x) => return Some(AstNode::Number(x)),
                _ => panic!("Invalid op: {:?}", op)
            }
        }
        else {
            panic!("Invalid op: {:?}", op);
        }
    }

    return ret;
}

/* Add a definition to the given state
 * TODO: add state struct
 */
fn add_define_to_state() -> () {
    return ();
}

/* Evaluate the given AST in place
 */
pub fn eval(ast: &mut AstNode) -> () {
    let mut result: Option<AstNode> = None;

    /* If expr
     *    determine if define or application
     */
    if let AstNode::Expression(ref mut expr) = *ast {
        // ref to boxed op, ref to slice of boxed args
        if let Some((op, args)) = (*expr).split_first_mut() {

            if let AstNode::Define = **op {
                // Define
                add_define_to_state();
            }
            else {
                // Application
                eval(&mut **op);
                for e in args.iter_mut() {
                    eval(&mut *e);
                }
                result = apply(op, args);
            }
    

        }
    }

    if let Some(x) = result {
        *ast = x;
    }

    return ();
}

#[cfg(test)]
mod test {
    use parser::AstNode;
    use eval::eval;

    #[test]
    fn simple_eval() {
        let mut ast = AstNode::Expression(vec![Box::new(
                           AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("+"))),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Number(4.0))]))]);
        eval(&mut ast);
        let expected_result = AstNode::Number(7.0);
        assert_eq!(ast, expected_result);
    }

    #[test]
    fn simple_sub() {
        let mut ast = AstNode::Expression(vec![Box::new(
                           AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("-"))),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Number(4.0))]))]);
        eval(&mut ast);
        let expected_result = AstNode::Number(-1.0);
        assert_eq!(ast, expected_result);
    }

    #[test]
    fn simple_mult() {
        let mut ast = AstNode::Expression(vec![Box::new(
                           AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("*"))),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Number(4.0))]))]);
        eval(&mut ast);
        let expected_result = AstNode::Number(12.0);
        assert_eq!(ast, expected_result);
    }

    #[test]
    fn simple_div() {
        let mut ast = AstNode::Expression(vec![Box::new(
                           AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("/"))),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Number(4.0))]))]);
        eval(&mut ast);
        let expected_result = AstNode::Number(0.75);
        assert_eq!(ast, expected_result);
    }

    #[test]
    fn nested_eval() {
        let mut ast = AstNode::Expression(vec![Box::new(
                           AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("+"))),
                                                 Box::new(AstNode::Number(3.0)),
                                                 Box::new(AstNode::Expression(vec![
                                                    Box::new(AstNode::Identifier(String::from("+"))),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Number(4.0))]))]))]);
        eval(&mut ast);
        let expected_result = AstNode::Number(10.0);
        assert_eq!(ast, expected_result);
    }

}
