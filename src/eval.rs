/* eval.rs
 *
 * Takes an AST and returns a result
 *
 */
use parser::AstNode;
use parser::F64Key;
use std::collections::BTreeMap;


fn reduce<F>(args: &[Box<AstNode>], f: F) -> AstNode
    where F: Fn(f64, f64) -> f64
{
    let mut sum: f64;
    match *args[0] {
        AstNode::Number(x) => sum = x.get(),
        _ => panic!("Invalid number arg: {:?}", args[0]),
    }
    for arg in args[1..].iter() {
        match **arg {
            AstNode::Number(x) => sum = f(x.get(), sum),
            _ => panic!("Invalid number arg: {:?}", arg),
        }
    }

    return AstNode::Number(F64Key::new(sum));
}

/* Apply the given evaluated arguments to the given operand */
fn apply(op: &AstNode, args: &[Box<AstNode>]) -> Option<AstNode> {
    let ret: Option<AstNode>;

    if let AstNode::Identifier(ref ident) = *op {
        match ident.as_str() {
            "+" => ret = Some(reduce(args, |x, sum| sum + x)),
            "*" => ret = Some(reduce(args, |x, prod| prod * x)),
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
 */
fn add_define_to_context(context: &mut BTreeMap<Box<AstNode>, Box<AstNode>>, name: &Box<AstNode>, value: &Box<AstNode>) -> () {

    if let AstNode::Identifier(_) = **name {
        context.insert((*name).clone(), (*value).clone());
    }
    else {
        panic!("The following must be an identifier {:?}", *name);
    }

    return ();
}

/* TODO: add lookup and tests */


/* Evaluate the given AST in place
 */
pub fn eval(ast: &mut AstNode) -> () {
    let mut result: Option<AstNode> = None;

    /* If expr
     *    determine if define or application
     */
    if let AstNode::Expression(ref mut expr) = *ast {
        // ref to boxed op, ref to slice of boxed args
        if let Some((p_op, args)) = (*expr).split_first_mut() {
            /* Handle the different types of build-int expressions
             *   Defines: bind a name to a value
             *   other identifier: evaluate the expr in the current context
             */
            match **p_op {
                AstNode::Define => {
                    // Define
                    if args.len() != 2 {
                        panic!("Can't assign multiple values to identifier {:?}", *args[0]);
                    }
					/* TODO: add in contexts*/
                    //add_define_to_context(context, &args[0], &args[1]);
                }
                _ => {
                    // Application
                    eval(&mut **p_op);
                    for e in args.iter_mut() {
                        eval(e);
                    }
                    result = apply(p_op, args);
                }
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
	use parser::F64Key;
    use eval::eval;

    #[test]
    fn simple_eval() {
        let mut ast = AstNode::Expression(vec![Box::new(
                           AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("+"))),
                                                    Box::new(AstNode::Number(F64Key::new(3.0))),
                                                    Box::new(AstNode::Number(F64Key::new(4.0)))]))]);
        eval(&mut ast);
        let expected_result = AstNode::Number(F64Key::new(7.0));
        assert_eq!(ast, expected_result);
    }

    #[test]
    fn simple_sub() {
        let mut ast = AstNode::Expression(vec![Box::new(
                           AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("-"))),
                                                    Box::new(AstNode::Number(F64Key::new(3.0))),
                                                    Box::new(AstNode::Number(F64Key::new(4.0)))]))]);
        eval(&mut ast);
        let expected_result = AstNode::Number(F64Key::new(-1.0));
        assert_eq!(ast, expected_result);
    }

    #[test]
    fn simple_mult() {
        let mut ast = AstNode::Expression(vec![Box::new(
                           AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("*"))),
                                                    Box::new(AstNode::Number(F64Key::new(3.0))),
                                                    Box::new(AstNode::Number(F64Key::new(4.0)))]))]);
        eval(&mut ast);
        let expected_result = AstNode::Number(F64Key::new(12.0));
        assert_eq!(ast, expected_result);
    }

    #[test]
    fn simple_div() {
        let mut ast = AstNode::Expression(vec![Box::new(
                           AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("/"))),
                                                    Box::new(AstNode::Number(F64Key::new(3.0))),
                                                    Box::new(AstNode::Number(F64Key::new(4.0)))]))]);
        eval(&mut ast);
        let expected_result = AstNode::Number(F64Key::new(0.75));
        assert_eq!(ast, expected_result);
    }

    #[test]
    fn nested_eval() {
        let mut ast = AstNode::Expression(vec![Box::new(
                           AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("+"))),
                                                 Box::new(AstNode::Number(F64Key::new(3.0))),
                                                 Box::new(AstNode::Expression(vec![
                                                    Box::new(AstNode::Identifier(String::from("+"))),
                                                    Box::new(AstNode::Number(F64Key::new(3.0))),
                                                    Box::new(AstNode::Number(F64Key::new(4.0)))]))]))]);
        eval(&mut ast);
        let expected_result = AstNode::Number(F64Key::new(10.0));
        assert_eq!(ast, expected_result);
    }

}
