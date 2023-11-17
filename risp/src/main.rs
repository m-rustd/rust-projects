use std::{io::{self, Write}, fmt, collections::HashMap, rc::Rc};

// Env
macro_rules! ensure_tonicity {
    ($check_fn:expr) => {{
        |args: &[RispExp]| -> Result<RispExp, RispError> {
            let floats = parse_list_of_floats(args)?;
            let first = floats.first().ok_or(RispError::Reason("expected at least one number".to_string()))?;
            let rest = &floats[1..];
            fn f(pre: &f64, rest: &[f64]) -> bool {
                match rest.first() {
                    Some(x) => $check_fn(pre, x) && f(x, &rest[1..]),
                    None => true,
                }
            }
            Ok(RispExp::Boolean(f(first, rest)))
        }
    }};
}

#[derive(Debug, Clone)]
enum RispExp {
    Boolean(bool),
    Number(f64),
    Symbol(String),
    List(Vec<RispExp>),
    Func(fn(&[RispExp]) -> Result<RispExp, RispError>),
    Lambda(RispLambda),
}

#[derive(Debug, Clone)]
struct RispLambda {
  params_exp: Rc<RispExp>,
  body_exp: Rc<RispExp>,
}

impl fmt::Display for RispExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            RispExp::Boolean(a) => a.to_string(),
            RispExp::Number(n) => n.to_string(),
            RispExp::Symbol(s) => s.clone(),
            RispExp::List(list) => {
                let s: Vec<String> = list.iter().map(|x|x.to_string()).collect();
                format!("({})", s.join(","))
            },
            RispExp::Func(_) => "Function {}".to_string(),
            RispExp::Lambda(_) => "Lambda {}".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug)]
enum RispError {
    Reason(String),
}

#[derive(Debug, Clone)]
struct RispEnv {
    data: HashMap<String, RispExp>,
    outer: Option<Box<RispEnv>>,
}
    
fn tokenize(input: &str) -> Vec<String> {
    input
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

// (+ 1 2 3)
fn parse(tokens: &[String]) -> Result<(RispExp, &[String]), RispError> {
    let (first, rest) = tokens.split_first().ok_or(RispError::Reason("could not get token".to_string()))?;
    match &first[..] {
        "(" => parse_expression(rest),
        ")" => Err(RispError::Reason("unexpected token)".to_string())),
        _ => Ok((parse_atom(first), rest)),
    }
}

// + 1 2 3)
fn parse_expression(tokens: &[String]) -> Result<(RispExp, &[String]), RispError> {
    let mut ret = Vec::new();
    let mut cur = tokens;
    loop {
        let peek = cur.get(0);
        if peek.is_some() {
            if peek.unwrap() == ")" {
                return Ok((RispExp::List(ret), &cur[1..]));
            }
        } else {
            return Err(RispError::Reason("can not found)".to_string()));
        }
        let (exp, rest) = parse(cur)?;
        ret.push(exp);
        cur = rest;
    }
}

// + 1 2 3
fn parse_atom(token: &str) -> RispExp {
    match token.as_ref() {
        "true" => RispExp::Boolean(true),
        "false" => RispExp::Boolean(false),
        _ => {
            match token.parse::<f64>() {
                Ok(n) => RispExp::Number(n),
                Err(_) => RispExp::Symbol(token.to_string()),
            }
        },
    }
}

fn parse_single_float(exp: &RispExp) -> Result<f64, RispError> {
    match exp {
        RispExp::Number(n) => Ok(*n),
        RispExp::Boolean(b) => Ok(*b as i32 as f64),
        _ => Err(RispError::Reason("expected a number".to_string())),
    }
}

fn parse_list_of_floats(args: &[RispExp]) -> Result<Vec<f64>, RispError> {
    args.iter().map(|x| parse_single_float(x)).collect()
}

fn init_env() -> RispEnv {
    let mut data: HashMap<String, RispExp> = HashMap::new();
    data.insert("+".to_string(), RispExp::Func(|args: &[RispExp]| -> Result<RispExp, RispError> {
        let sum = parse_list_of_floats(args)?.iter().sum();
        Ok(RispExp::Number(sum))
    }));
    data.insert("-".to_string(), RispExp::Func(|args: &[RispExp]| -> Result<RispExp, RispError> {
        let floats = parse_list_of_floats(args)?;
        let first = floats.first().ok_or(RispError::Reason("expected at least one number".to_string()))?;
        let rest_sum: f64 = floats[1..].iter().sum();
        Ok(RispExp::Number(first - rest_sum))
    }));
    data.insert(
      "=".to_string(), 
      RispExp::Func(ensure_tonicity!(|a, b| a == b))
    );
    data.insert(
      ">".to_string(), 
      RispExp::Func(ensure_tonicity!(|a, b| a > b))
    );
    data.insert(
      ">=".to_string(), 
      RispExp::Func(ensure_tonicity!(|a, b| a >= b))
    );
    data.insert(
      "<".to_string(), 
      RispExp::Func(ensure_tonicity!(|a, b| a < b))
    );
    data.insert(
      "<=".to_string(), 
      RispExp::Func(ensure_tonicity!(|a, b| a <= b))
    );
    RispEnv { data, outer: None }
}

// map get
fn env_get(k: &str, env: &RispEnv) -> Option<RispExp> {
    match env.data.get(k) {
        Some(v) => Some(v.clone()),
        None => {
            match &env.outer {
                Some(o) => env_get(k, o),
                None => None
            }
        },
    }
}

// (if false 1 2) -> 2
fn eval_if_args(arg_forms: &[RispExp], env: &mut RispEnv) -> Result<RispExp, RispError> {
    let test_form = arg_forms.first().ok_or(RispError::Reason("expected test form".to_string()))?;
    let test_eval = eval(test_form, env)?;
    match test_eval {
        RispExp::Boolean(b) => {
            let form_idx = if b { 1 } else { 2 };
            let res_form = arg_forms.get(form_idx)
              .ok_or(RispError::Reason(
                format!("expected form idx={}", form_idx)
              ))?;
            let res_eval = eval(res_form, env);
            res_eval
        },
        _ => Err(RispError::Reason(format!("unexpected test form='{}'", test_form.to_string())))
    }
}

// def a 1 => (+ a 1)
fn eval_def_args(arg_forms: &[RispExp], env: &mut RispEnv) -> Result<RispExp, RispError> {
    let first_form = arg_forms.first().ok_or(RispError::Reason("expected first form".to_string()))?;
    let first_str = match first_form {
        RispExp::Symbol(s) => Ok(s.clone()),
        _ => Err(RispError::Reason(
          "expected first form to be a symbol".to_string(),
        ))
    }?;
    let second_form = arg_forms.get(1).ok_or(
      RispError::Reason(
        "expected second form".to_string(),
      )
    )?;
    if arg_forms.len() > 2 {
      return Err(
        RispError::Reason(
          "def can only have two forms ".to_string(),
        )
      )
    }
    let second_eval = eval(second_form, env)?;
    env.data.insert(first_str, second_eval);
    Ok(first_form.clone())
}

fn eval_lambda_args(arg_forms: &[RispExp]) -> Result<RispExp, RispError> {
    let params_exp = arg_forms.first().ok_or(
      RispError::Reason(
        "expected args form".to_string(),
      )
    )?;
    let body_exp = arg_forms.get(1).ok_or(
      RispError::Reason(
        "expected second form".to_string(),
      )
    )?;
    if arg_forms.len() > 2 {
      return Err(
        RispError::Reason(
          "fn definition can only have two forms ".to_string(),
        )
      )
    }
    Ok(
      RispExp::Lambda(
        RispLambda {
          body_exp: Rc::new(body_exp.clone()),
          params_exp: Rc::new(params_exp.clone()),
        }
      )
    )
}

fn parse_list_of_symbol_strings(form: Rc<RispExp>) -> Result<Vec<String>, RispError> {
    let list = match form.as_ref() {
        RispExp::List(s) => Ok(s.clone()),
        _ => Err(RispError::Reason(
          "expected args form to be a list".to_string(),
        ))
    }?;
    list.iter().map(|x| {
        match x {
            RispExp::Symbol(s) => Ok(s.clone()),
            _ => Err(RispError::Reason(
              "expected symbols in the argument list".to_string(),
            ))
        }
    }).collect()
}

fn env_for_lambda(params: Rc<RispExp>, arg_forms: &[RispExp], outer_env: &mut RispEnv) -> Result<RispEnv, RispError> {
    let ks = parse_list_of_symbol_strings(params)?;
    if ks.len() != arg_forms.len() {
      return Err(
        RispError::Reason(
          format!("expected {} arguments, got {}", ks.len(), arg_forms.len())
        )
      );
    }
    let vs = eval_forms(arg_forms, outer_env)?;
    let mut data: HashMap<String, RispExp> = HashMap::new();
    for (k, v) in ks.iter().zip(vs.iter()) {
        data.insert(k.clone(), v.clone());
    }
    Ok(
      RispEnv {
        data,
        outer: Some(Box::new(outer_env.clone())),
      }
    )
}

// if def fn
fn eval_built_in_form(exp: &RispExp, arg_forms: &[RispExp], env: &mut RispEnv) -> Option<Result<RispExp, RispError>> {
    match exp {
        RispExp::Symbol(s) => {
            match s.as_ref() {
                "if" => Some(eval_if_args(arg_forms, env)),
                "def" => Some(eval_def_args(arg_forms, env)),
                "fn" => Some(eval_lambda_args(arg_forms)),
                _ => None,
            }
        },
        _ => None,
    }
}

// 执行list中所有计算
// + 1 2 3
fn eval_forms(arg_forms: &[RispExp], env: &mut RispEnv) -> Result<Vec<RispExp>, RispError> {
    arg_forms.iter().map(|x| eval(x, env)).collect()
}

// 执行计算
fn eval(exp: &RispExp, env: &mut RispEnv) -> Result<RispExp, RispError> {
    match exp {
        RispExp::Boolean(_) => Ok(exp.clone()),
        RispExp::Number(_) => Ok(exp.clone()),
        RispExp::Symbol(op) => {
            env_get(op, env).ok_or(RispError::Reason(format!("unexpected symbol k='{}'", op)))
        },
        RispExp::Func(_) => Err(RispError::Reason("unexpected function".to_string())),
        RispExp::Lambda(_) => Err(RispError::Reason("unexpected form".to_string())),
        RispExp::List(list) => {
            // + 1 2
            let first_form = list.first().ok_or(RispError::Reason("expected a non empty list".to_string()))?;
            let arg_forms = &list[1..];
            match eval_built_in_form(first_form, arg_forms, env) {
                Some(res) => res,
                None => {
                    let first_eval = eval(first_form, env)?;
                    match first_eval {
                        RispExp::Func(f) => {
                            f(&eval_forms(arg_forms, env)?)
                        },
                        RispExp::Lambda(lambda) => {
                            let new_env = &mut env_for_lambda(lambda.params_exp, arg_forms, env)?;
                            eval(&lambda.body_exp, new_env)
                        }
                        _ => Err(RispError::Reason("first form must be a function".to_string()))
                    }
                },
            }
        },
    }
}

fn parse_eval(expr: &str, env: &mut RispEnv) -> Result<RispExp, RispError> {
    let tokens: Vec<String> = tokenize(expr);
    println!("tokens: {:?}", tokens);
    let (exp, _) = parse(&tokens)?;
    println!("exp: {:?}", exp);
    eval(&exp, env)
}

fn main() {
    let mut env = init_env();
    let mut input = String::new();
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.len() == 0 {
            break;
        }
        match parse_eval(&input, &mut env) {
            Ok(res) => println!("// 🔥 => {}", res),
            Err(e) => match e {
                RispError::Reason(msg) => println!("// 🙀 => {}", msg),
            },
        }
        input.clear();
    }
}