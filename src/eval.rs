/* eval.rs
 *
 * Takes an AST and returns a result
 *
 */
use parser::AstNode;
use std::collections::BTreeMap;


pub struct Context {
    defines: BTreeMap<String, Box<AstNode>>,
}

impl Context {
    /* Add a definition to the given state
    */
    pub fn new() -> Context {
        return Context{defines: BTreeMap::new()};
    }
    pub fn add_define(&mut self, name: Box<AstNode>, value: Box<AstNode>) -> () {

        if let AstNode::Identifier(ref identifier) = *name {
            (*self).defines.insert((identifier.clone()), value);
        }
        else {
            panic!("The following must be an identifier {:?}", *name);
        }

        return ();
    }
    fn get_define(&mut self, name: &Box<AstNode>) -> Option<&Box<AstNode>> {

        if let AstNode::Identifier(ref identifier) = **name {
            return (*self).defines.get(identifier);
        }
        else {
            panic!("The following must be an identifier {:?}", *name);
        }
    }
}

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

    return AstNode::Number((sum));
}

/* TODO: add lookup and evaluation of defines */
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


/* Evaluate the given AST in place
 */
pub fn eval(ast: &mut AstNode, context: &mut Context) -> () {
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
                /* TODO: delete after define work is done
                AstNode::Define => {
                    // Define
                    if args.len() != 2 {
                        panic!("Can't assign multiple values to identifier {:?}", *args[0]);
                    }
                    context.add_define(args[0].clone(), args[1].clone());
                }
                */
                _ => {
                    // Application
                    eval(&mut **p_op, context);
                    for e in args.iter_mut() {
                        eval(e, context);
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
    use eval::eval;
    use eval::Context;

    #[test]
    fn simple_context() {
        let mut c = Context::new();
        let name = Box::new(AstNode::Identifier(String::from("A")));
        let same_name = Box::new(AstNode::Identifier(String::from("A")));
        let value = Box::new(AstNode::Number(10.0));
        c.add_define(name.clone(), value.clone());
        assert_eq!(value, *c.get_define(&same_name).unwrap());
    }

    #[test]
    fn context_get_non_existent_define() {
        let mut c = Context::new();
        let name = Box::new(AstNode::Identifier(String::from("A")));
        let undefined = Box::new(AstNode::Identifier(String::from("BLAH")));
        let value = Box::new(AstNode::Number(10.0));
        c.add_define(name.clone(), value.clone());
        assert_eq!(None, c.get_define(&undefined));
    }

    #[test]
    fn simple_eval() {
        let mut c = Context::new();
        let mut ast = AstNode::Expression(vec![Box::new(
                           AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("+"))),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Number(4.0))]))]);
        eval(&mut ast, &mut c);
        let expected_result = AstNode::Number(7.0);
        assert_eq!(ast, expected_result);
    }

    #[test]
    fn simple_sub() {
        let mut c = Context::new();
        let mut ast = AstNode::Expression(vec![Box::new(
                           AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("-"))),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Number(4.0))]))]);
        eval(&mut ast, &mut c);
        let expected_result = AstNode::Number((-1.0));
        assert_eq!(ast, expected_result);
    }

    #[test]
    fn simple_mult() {
        let mut c = Context::new();
        let mut ast = AstNode::Expression(vec![Box::new(
                           AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("*"))),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Number(4.0))]))]);
        eval(&mut ast, &mut c);
        let expected_result = AstNode::Number(12.0);
        assert_eq!(ast, expected_result);
    }

    #[test]
    fn simple_div() {
        let mut c = Context::new();
        let mut ast = AstNode::Expression(vec![Box::new(
                           AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("/"))),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Number(4.0))]))]);
        eval(&mut ast, &mut c);
        let expected_result = AstNode::Number((0.75));
        assert_eq!(ast, expected_result);
    }

    #[test]
    fn nested_eval() {
        let mut c = Context::new();
        let mut ast = AstNode::Expression(vec![Box::new(
                           AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("+"))),
                                                 Box::new(AstNode::Number(3.0)),
                                                 Box::new(AstNode::Expression(vec![
                                                    Box::new(AstNode::Identifier(String::from("+"))),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Number(4.0))]))]))]);
        eval(&mut ast, &mut c);
        let expected_result = AstNode::Number(10.0);
        assert_eq!(ast, expected_result);
    }

}
