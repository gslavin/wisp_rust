/* eval.rs
 *
 * Takes an AST and returns a result
 *
 */
use parser::AstNode;

fn add(args: &[Box<AstNode>]) -> AstNode
{
    let mut sum: f64 = 0.0;
    for arg in args.iter() {
        match **arg {
            AstNode::Number(x) => sum += x,
            _ => panic!("Invalid number arg: {:?}", arg),
        }
    }

    return AstNode::Number(sum);
}

fn apply(op: &AstNode, args: &[Box<AstNode>]) -> Option<AstNode>
{
    let ret: Option<AstNode>;

    if let AstNode::Identifier(ref ident) = *op {
        match ident.as_str() {
            "+" => ret = Some(add(args)),
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

pub fn eval(ast: &mut AstNode) -> ()
{
    let mut result: Option<AstNode> = None;

    if let AstNode::Expression(ref mut expr) = *ast {
        for e in expr.iter_mut() {
            eval(&mut *e);
        }
        if let Some((op, args)) = (*expr).split_first() {
            result = apply(op, args);
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
