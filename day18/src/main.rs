use nom::{
    character::complete::{multispace0, one_of},
    combinator::{map_res, recognize, value},
    error::ParseError,
    multi::{many0, many1_count},
    sequence::{delimited, terminated},
    IResult,
};
use std::collections::VecDeque;

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn decimal(input: &str) -> IResult<&str, u64> {
    map_res(recognize(many1_count(one_of("0123456789"))), |out: &str| {
        out.parse()
    })(input)
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Plus,
    Times,
}

impl Operator {
    fn apply(&self, x: u64, y: u64) -> u64 {
        use Operator::*;
        (match self {
            Plus => std::ops::Add::add,
            Times => std::ops::Mul::mul,
        })(x, y)
    }
}

fn op(input: &str) -> IResult<&str, Operator> {
    nom::branch::alt((
        value(Operator::Plus, nom::character::complete::char('+')),
        value(Operator::Times, nom::character::complete::char('*')),
    ))(input)
}

#[derive(Debug, Clone, Copy)]
enum Term {
    Litteral(u64),
    Op(Operator),
}

fn op_ws_term(input: &str) -> IResult<&str, Term> {
    nom::combinator::map(ws(op), Term::Op)(input)
}

fn li_or_subex_term(input: &str) -> IResult<&str, Term> {
    nom::combinator::map(
        nom::branch::alt((
            decimal,
            nom::sequence::delimited(
                nom::character::complete::char('('),
                evaluate,
                nom::character::complete::char(')'),
            ),
        )),
        Term::Litteral,
    )(input)
}

fn evaluate(input: &str) -> IResult<&str, u64> {
    nom::combinator::flat_map(li_or_subex_term, |x0| {
        nom::combinator::map(
            nom::multi::fold_many1(
                nom::branch::alt((li_or_subex_term, op_ws_term)),
                VecDeque::new(),
                |mut acc, v| {
                    acc.push_back(v);
                    acc
                },
            ),
            move |mut que: VecDeque<Term>| {
                let mut v0: u64 = match x0 {
                    Term::Litteral(v) => v,
                    _ => panic!(),
                };
                let mut res = 1;
                loop {
                    let last_op = match que.pop_front() {
                        Some(Term::Op(o)) => o,
                        Some(_) => panic!(),
                        None => break,
                    };
                    match que.pop_front() {
                        Some(Term::Litteral(v)) => match last_op {
                            Operator::Plus => v0 += v,
                            Operator::Times => {
                                res *= v0;
                                v0 = v
                            }
                        },
                        _ => panic!(),
                    }
                }
                res * v0
            },
        )
    })(input)
    // nom::combinator::map(
    //     nom::sequence::tuple(
    //         (decimal, ws(op), nom::branch::alt((evaluate, decimal)))
    //     ),
    //     |(x, op, y)| op.apply(x, y),
    // )(input)
}

fn main() -> anyhow::Result<()> {
    let buf = String::from_utf8(std::fs::read("input.txt")?)?;
    let a1: u64 = buf.lines().map(|s| evaluate(s).unwrap().1).sum();
    println!("{}", a1);
    Ok(())
}
