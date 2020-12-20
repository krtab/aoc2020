use combine::{
    any, between, choice, from_str,
    parser::{
        char::{char, digit, space, string},
        range::recognize,
    },
    sep_by, sep_end_by, skip_many1, EasyParser, Parser,
};
use regex::Regex;
use vec_map::VecMap as Map;


type T = u64;

fn build_regex(rules: &Map<Rule>, id: usize, cache: &mut Map<String>) -> String {
    if let Some(s) = cache.get(id) {
        s.clone()
    } else {
        let v = match rules.get(id).unwrap() {
            Rule::Litteral(c) => {
                let mut res = String::new();
                res.push(*c);
                res
            }
            Rule::Disjunction(clauses) => {
                let mut res = String::new();
                res.push('(');
                let mut first = true;
                for cl in clauses {
                    if !first {
                        res.push('|');
                    };
                    for term in cl {
                        res.push_str(&build_regex(rules, *term as usize, cache));
                    }
                    first = false;
                }
                res.push(')');
                res
            }
        };
        cache.insert(id, v.clone());
        v
    }
}

fn decimal<'a>() -> impl EasyParser<&'a str, Output = T> {
    from_str::<_, T, _>(recognize(skip_many1(digit())))
}

#[derive(Debug, Clone)]
enum Rule {
    Litteral(char),
    Disjunction(Vec<Vec<T>>),
}

fn main() -> anyhow::Result<()> {
    let buf = String::from_utf8(std::fs::read("input.txt")?)?;
    let mut parse_rule = (
        decimal().skip(char(':')).skip(space()),
        choice((
            between(char('"'), char('"'), any()).map(Rule::Litteral),
            sep_by(sep_end_by(decimal(), space()), string("| ")).map(Rule::Disjunction),
        )),
    );
    let mut rules = Map::new();
    let mut messages = Vec::new();
    let mut iter = buf.lines();
    for l in &mut iter {
        match parse_rule.easy_parse(l) {
            Ok(((rule_id, rule_body), _)) => {
                rules.insert(rule_id as usize, rule_body);
            }
            Err(_) => break,
        };
    }
    for l in &mut iter {
        messages.push(l);
    }
    let mut cache = Map::new();
    let mut re = String::new();
    re.push('^');
    re.push_str(&build_regex(&rules, 0, &mut cache));
    re.push('$');
    let re = Regex::new(&re)?;
    // for (i,msg) in messages.iter().enumerate() {
    //     println!("{}: {}",i, re.is_match(msg));
    // }
    let a1 = messages.iter().filter(|&msg| re.is_match(msg)).count();
    println!("Answer1: {}", a1);
    Ok(())
}
