/* eval.rs
 *
 * Takes an AST and returns a result
 *
 */
use parser::AstNode;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct Context {
    defines: Vec<BTreeMap<String, Box<AstNode>>>,
}

impl Context {
    /* Add a definition to the given state */
    pub fn new() -> Context {
        return Context{defines: vec![BTreeMap::new()]};
    }
    pub fn add_namespace(&mut self) -> () {
        (*self).defines.push(BTreeMap::new())
    }
    pub fn remove_namespace(&mut self) -> () {
        if (*self).defines.len() == 0 {
            panic!("Can't remove only namespace!");
        }
        (*self).defines.pop();
    }
    pub fn add_define(&mut self, name: String, value: Box<AstNode>) -> Option<Box<AstNode>> {
        let end = (*self).defines.len() - 1;
        return (*self).defines[end].insert(name, value);
    }
    fn get_define(&self, name: &String) -> Option<&Box<AstNode>> {
        for defines in (*self).defines.iter().rev() {
            if let Some(value) = defines.get(name) {
                return Some(value);
            }
        }

        return None;
    }
}

fn is_builtin(ident: &String) -> bool {
    lazy_static! {
        static ref BUILTINS: BTreeSet<&'static str> = ["+", "-", "*", "/"].iter().cloned().collect();
    }
    return BUILTINS.contains(&ident.as_str());
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

/* Apply the given evaluated arguments to the given operand */
fn apply(op: &AstNode, args: &[Box<AstNode>], context: &mut Context) -> AstNode {
    match *op {
        AstNode::Identifier(ref ident) => {
            match ident.as_str() {
                "+" => reduce(args, |x, sum| sum + x),
                "*" => reduce(args, |x, prod| prod * x),
                "-" => reduce(args, |x, sum| sum - x),
                "/" => reduce(args, |x, prod| prod / x),
                _ => {
                    /* TODO: implement functions
                       match context.get_define(&String::from(name)) {
                       Some(value) => *(value.clone()),
                       None => panic!("Undefined Identifier: {:?}", name)
                       }
                       */
                    panic!("TODO: implement functions")
                }
            }
        },
        // TODO: avoid copying when creating sub context
        AstNode::Lambda(ref parameters, ref expr) => {
            // Add arg values to context
            (*context).add_namespace();
            for (param, arg) in parameters.iter().zip(args.iter()) {
                context.add_define(param.clone(), arg.clone());
            }
            // Eval expression
            let mut lambda_body = (**expr).clone();
            eval(&mut lambda_body, context);
            (*context).remove_namespace();
            return lambda_body;
        },
        ref op => panic!("Invalid operator: {:?}", op)
    }
}

/* Evaluate the given AST in place
 */
pub fn eval(ast: &mut AstNode, context: &mut Context) -> () {
    let mut result: Option<AstNode> = None;

    match *ast {
        AstNode::Define(ref name, ref value) => {
            if !is_builtin(name) {
                context.add_define(name.clone(), value.clone());
            }
            else {
                panic!("Can't override buildin: {:?}", name);
            }
        },
        AstNode::Expression(ref mut expr) => {
            if let Some((p_op, args)) = (*expr).split_first_mut() {
                // Evaluate operator
                eval(&mut **p_op, context);
                // Evaluate all arguments
                for e in args.iter_mut() {
                    eval(e, context);
                }
                result = Some(apply(p_op, args, context));
            }

        },
        AstNode::Identifier(ref ident) => {
            // substitute defines
            if !is_builtin(ident) {
                match context.get_define(ident) {
                    Some(value) => result = Some((**value).clone()),
                    None => panic!("Undefined Identifier: {:?}", ident)
                }
            }
        },
        AstNode::If(ref mut pred, ref mut true_expr, ref mut false_expr) => {
            eval(pred, context);
            match **pred {
                AstNode::Bool(b) => {
                    if b {
                        eval(true_expr, context);
                        result = Some((**true_expr).clone());
                    }
                    else {
                        eval(false_expr, context);
                        result = Some((**false_expr).clone());
                    }
                },
                ref pred => panic!("Unexpected predicate for if statement: {:?}", pred)
            }
        },
        _ => {}
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
        let name = String::from("A");
        let value = Box::new(AstNode::Number(10.0));
        c.add_define(name.clone(), value.clone());
        assert_eq!(value, *c.get_define(&name).unwrap());
    }

    #[test]
    fn multiple_namespace_context() {
        let mut c = Context::new();
        let name = String::from("A");
        let old_value = Box::new(AstNode::Number(5.0));
        let value = Box::new(AstNode::Number(10.0));
        c.add_define(name.clone(), old_value.clone());
        c.add_namespace();
        c.add_define(name.clone(), value.clone());
        assert_eq!(value, *c.get_define(&name).unwrap());
        c.remove_namespace();
        assert_eq!(old_value, *c.get_define(&name).unwrap());
    }

    #[test]
    fn context_get_non_existent_define() {
        let mut c = Context::new();
        let name = String::from("A");
        let undefined = String::from("BLAH");
        let value = Box::new(AstNode::Number(10.0));
        c.add_define(name.clone(), value.clone());
        assert_eq!(None, c.get_define(&undefined));
    }

    #[test]
    fn eval_simple_define() {
        let mut c = Context::new();
        let name = String::from("A");
        let value = Box::new(AstNode::Number(10.0));
        c.add_define(name.clone(), value.clone());
        assert_eq!(value, *c.get_define(&name).unwrap());

        let mut ast = AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("+"))),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Identifier(String::from("A")))]);
        eval(&mut ast, &mut c);
        let expected_result = AstNode::Number(13.0);
        assert_eq!(ast, expected_result);
    }

    #[test]
    fn eval_operator_define() {
        let mut c = Context::new();
        let name = String::from("ADD");
        let value = Box::new(AstNode::Identifier(String::from("+")));
        c.add_define(name.clone(), value.clone());
        assert_eq!(value, *c.get_define(&name).unwrap());

        let mut ast = AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("ADD"))),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Number(4.0))]);
        eval(&mut ast, &mut c);
        let expected_result = AstNode::Number(7.0);
        assert_eq!(ast, expected_result);
    }

    #[test]
    fn simple_lambda() {
        // ((lambda (x) (* x x)) 4)
        let mut c = Context::new();
        let mut ast = AstNode::Expression(vec![
                            Box::new(AstNode::Lambda(vec![String::from("x")],
                                            Box::new(AstNode::Expression(vec![
                                                 Box::new(AstNode::Identifier(String::from("*"))),
                                                 Box::new(AstNode::Identifier(String::from("x"))),
                                                 Box::new(AstNode::Identifier(String::from("x")))])))),
                            Box::new(AstNode::Number(4.0))]);
        eval(&mut ast, &mut c);
        let expected_result = AstNode::Number(16.0);
        assert_eq!(ast, expected_result);
    }

    #[test]
    fn simple_eval() {
        let mut c = Context::new();
        let mut ast = AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("+"))),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Number(4.0))]);
        eval(&mut ast, &mut c);
        let expected_result = AstNode::Number(7.0);
        assert_eq!(ast, expected_result);
    }

    #[test]
    fn simple_sub() {
        let mut c = Context::new();
        let mut ast = AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("-"))),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Number(4.0))]);
        eval(&mut ast, &mut c);
        let expected_result = AstNode::Number((-1.0));
        assert_eq!(ast, expected_result);
    }

    #[test]
    fn simple_mult() {
        let mut c = Context::new();
        let mut ast = AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("*"))),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Number(4.0))]);
        eval(&mut ast, &mut c);
        let expected_result = AstNode::Number(12.0);
        assert_eq!(ast, expected_result);
    }

    #[test]
    fn simple_div() {
        let mut c = Context::new();
        let mut ast = AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("/"))),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Number(4.0))]);
        eval(&mut ast, &mut c);
        let expected_result = AstNode::Number((0.75));
        assert_eq!(ast, expected_result);
    }

    #[test]
    fn nested_eval() {
        let mut c = Context::new();
        let mut ast = AstNode::Expression(vec![Box::new(AstNode::Identifier(String::from("+"))),
                                                 Box::new(AstNode::Number(3.0)),
                                                 Box::new(AstNode::Expression(vec![
                                                    Box::new(AstNode::Identifier(String::from("+"))),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Number(4.0))]))]);
        eval(&mut ast, &mut c);
        let expected_result = AstNode::Number(10.0);
        assert_eq!(ast, expected_result);
    }

    #[test]
    fn simple_if_true() {
        let mut c = Context::new();
        let mut ast = AstNode::If(Box::new(AstNode::Bool(true)),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Number(4.0)));
        eval(&mut ast, &mut c);
        let expected_result = AstNode::Number((3.0));
        assert_eq!(ast, expected_result);
    }

    #[test]
    fn simple_if_false() {
        let mut c = Context::new();
        let mut ast = AstNode::If(Box::new(AstNode::Bool(false)),
                                                    Box::new(AstNode::Number(3.0)),
                                                    Box::new(AstNode::Number(4.0)));
        eval(&mut ast, &mut c);
        let expected_result = AstNode::Number((4.0));
        assert_eq!(ast, expected_result);
    }
}
