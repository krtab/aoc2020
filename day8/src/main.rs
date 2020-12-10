mod instr {
    use std::error;
    use std::{
        fmt::{Debug, Display},
        str::FromStr,
    };
    #[derive(Debug)]
    pub enum Error {
        ParseArgError {
            arg: String,
            err: <i64 as FromStr>::Err,
        },
        UnknownInstr(String),
    }

    impl Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use Error::*;
            match self {
                ParseArgError { arg: s, err: _ } => {
                    f.write_fmt(format_args!("Couldn't parse argmuent {}.", s))
                }
                UnknownInstr(s) => f.write_fmt(format_args!("Unkown instruction: {}", s)),
            }
        }
    }
    impl error::Error for Error {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            use Error::*;
            match self {
                ParseArgError { arg: _, err: e } => Some(e),
                UnknownInstr(_) => None,
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Instr {
        Acc(i64),
        Jump(i64),
        Noop(i64),
    }

    impl FromStr for Instr {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (instr_str, arg_str) = s.split_at(3);
            let arg_str = &arg_str[1..];
            let arg: i64 = match arg_str.parse() {
                Ok(x) => x,
                Err(err) => {
                    return Err(Error::ParseArgError {
                        arg: arg_str.to_string(),
                        err,
                    })
                }
            };
            match instr_str {
                "acc" => Ok(Instr::Acc(arg)),
                "jmp" => Ok(Instr::Jump(arg)),
                "nop" => Ok(Instr::Noop(arg)),
                _ => Err(Error::UnknownInstr(instr_str.to_string())),
            }
        }
    }
}

use instr::Instr;

#[derive(Debug, Clone, Copy)]
enum InterpreterError {
    InfiniteLoop,
}

#[derive(Debug, Clone)]
struct State {
    already_executed: Vec<bool>,
    instr_ptr: usize,
    acc: i64,
}

impl State {
    fn new(program_size: usize) -> Self {
        Self {
            already_executed: vec![false; program_size],
            instr_ptr: 0,
            acc: 0,
        }
    }

    fn step(&mut self, instruction: Instr) -> Result<(), InterpreterError> {
        let test = self.already_executed.get_mut(self.instr_ptr).unwrap();
        if *test {
            return Err(InterpreterError::InfiniteLoop);
        } else {
            *test = true;
            use Instr::*;
            match &instruction {
                Noop(_) => Ok(self.instr_ptr += 1),
                Acc(x) => Ok({
                    self.instr_ptr += 1;
                    self.acc += x;
                }),
                Jump(offset) => Ok(self.instr_ptr = ((self.instr_ptr as i64) + offset) as usize),
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let buf = String::from_utf8(std::fs::read("input.txt")?)?;
    let program: Result<Vec<Instr>, _> = buf.lines().map(|s| s.parse()).collect();
    let program = program?;
    {
        let mut state = State::new(program.len());
        while let Ok(_) = state.step(program[state.instr_ptr]) {}
        println!("Answer1: {}", state.acc);
    }
    {
        let mut queue = vec![(false,State::new(program.len()))];
        while let Some((already_changed,mut state)) = queue.pop() {
            if state.instr_ptr == program.len() {
                println!("Answer2: {}",state.acc);
                continue
            }
            let instr = program[state.instr_ptr];
            use Instr::*;
            if !already_changed {
                let flipped_instr = match instr {
                    Acc(_) => None,
                    Noop(arg) => Some(Jump(arg)),
                    Jump(arg) => Some(Noop(arg)),
                };
                if let Some(flipped_instr) = flipped_instr {
                    let mut new_state = state.clone();
                    match new_state.step(flipped_instr) {
                        Ok(_) => queue.push((true,new_state)),
                        Err(InterpreterError::InfiniteLoop) => (),
                    }
                }
            }
            match state.step(instr) {
                Ok(_) => queue.push((already_changed,state)),
                Err(InterpreterError::InfiniteLoop) => (),
            }
        }
    }
    Ok(())
}
