use crate::args::{Args, SpecialMode};
use std::env;

/// 解析器，负责将命令行参数解析成对应类型的参数集。
#[derive(Debug)]
pub struct Parser {}

impl Parser {
    /// 创建一个新的解析器。
    pub fn new() -> &'static Parser {
        &Parser {}
    }

    /// 解析命令行参数
    pub fn parse(&self, rawargs: env::ArgsOs) -> ParseResult<Args> {
        let mut args = Args::default();
        let mut p = lexopt::Parser::from_args(rawargs);
        let mut v: Vec<String> = vec![];
        while let Some(arg) = p.next().unwrap().take() {
            match arg {
                lexopt::Arg::Short(short) if short == 'h' => {
                    args.special = Some(SpecialMode::HelpShort);
                }
                lexopt::Arg::Short(short) => {
                    return ParseResult::Err(anyhow::anyhow!("unknow {}", short));
                }
                lexopt::Arg::Long(long) if long == "help" => {
                    args.special = Some(SpecialMode::HelpLong);
                }
                lexopt::Arg::Long(long) => {
                    return ParseResult::Err(anyhow::anyhow!("unknow {}", long));
                }
                lexopt::Arg::Value(value) => {
                    v.push(value.into_string().unwrap());
                }
            }
        }

        if v.len() < 2 {
            args.special = Some(SpecialMode::HelpShort);
        }

        if let Some(special) = args.special.take() {
            return ParseResult::Special(special);
        }

        args.patterns = v[1].clone();
        args.positional = v[2].clone();
        ParseResult::Ok(args)
    }
}

/// 定义命令行解析结果
pub enum ParseResult<T> {
    Special(SpecialMode),
    Ok(T),
    Err(anyhow::Error),
}
