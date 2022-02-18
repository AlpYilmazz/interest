
use lazy_static::lazy_static;
use regex::Regex;
use crate::{expr, interpreter::ValueType, bytecode::CodeContext};

static PARSE_REGEX: &'static str = r#"^(?P<keyword>\S+)( (?P<value>\d+$)| ('(?P<name>\S+)')){0,1}$"#;

#[derive(Default)]
pub struct ParseEngine {}
impl ParseEngine {
    pub fn parse<'a>(&self, expr_str: &str, context: &mut CodeContext) -> Box<dyn expr::Expr> {

        let line = context.line();

        lazy_static! {
            static ref RE: Regex = Regex::new(PARSE_REGEX).unwrap();
        }
        let captures = RE.captures(expr_str);

        let expr: Box<dyn expr::Expr>;
        match captures {
            Some(capture) => {
                let keyword = capture.name("keyword").unwrap().as_str();
                let value = capture.name("value");
                let name = capture.name("name");

                expr = match keyword {
                    "START" => {
                        assert!(matches!(value, None));
                        assert!(matches!(name, None));
                        context.set_start(line);
                        Box::new(expr::Start {})
                    },
                    "LOAD_VAL" => {
                        assert!(matches!(name, None));
                        let value 
                            = ValueType::from_str_radix(value.unwrap().as_str(), 10)
                                .unwrap();
                        Box::new(expr::LoadVal { literal: value })
                    },
                    "WRITE_VAR" => {
                        assert!(matches!(value, None));
                        let name = name.unwrap().as_str().to_owned();
                        Box::new(expr::WriteVar { var_name: name })
                    },
                    "READ_VAR" => {
                        assert!(matches!(value, None));
                        let name = name.unwrap().as_str().to_owned();
                        Box::new(expr::ReadVar { var_name: name })
                    },
                    "ADD" => {
                        assert!(matches!(value, None));
                        assert!(matches!(name, None));
                        Box::new(expr::Add {})
                    },
                    "MULTIPLY" => {
                        assert!(matches!(value, None));
                        assert!(matches!(name, None));
                        Box::new(expr::Multiply {})
                    },
                    "LABEL" => {
                        assert!(matches!(value, None));
                        let name = name.unwrap().as_str();
                        context.set_label(name, line+1);
                        Box::new(expr::Label {})
                    },
                    "CALL" => {
                        assert!(matches!(value, None));
                        let name = name.unwrap().as_str().to_owned();
                        Box::new(expr::Call {
                            func_name: name,
                            return_line: line+1
                        })
                    },
                    "RETURN_VALUE" => {
                        assert!(matches!(value, None));
                        assert!(matches!(name, None));
                        Box::new(expr::ReturnValue {})
                    },
                    "RETURN" => {
                        assert!(matches!(value, None));
                        assert!(matches!(name, None));
                        Box::new(expr::Return {})
                    },
                    "JUMP" => {
                        assert!(matches!(value, None));
                        let name = name.unwrap().as_str().to_owned();
                        Box::new(expr::flow::Jump { label: name })
                    },
                    "JUMP_ZERO" => {
                        assert!(matches!(value, None));
                        let name = name.unwrap().as_str().to_owned();
                        Box::new(expr::flow::JumpZero { label: name })
                    },
                    "LOOP" => {
                        assert!(matches!(value, None));
                        assert!(matches!(name, None));

                        context.push_loop();
                        Box::new(expr::loops::Loop {
                            line: line,
                            loop_var: None,
                            count: None,
                            endloop: None,
                        })
                    },
                    "ENDLOOP" => {
                        assert!(matches!(value, None));
                        assert!(matches!(name, None));
                        context.consume_loop();
                        Box::new(expr::loops::EndLoop {
                            line: line,
                            loop_var: None,
                            loopstart: None,
                        })
                    },
                    "WHILE" => {
                        assert!(matches!(value, None));
                        assert!(matches!(name, None));
                        context.push_while();
                        Box::new(expr::loops::While {
                            line: line,
                            endwhile: None,
                        })
                    },
                    "ENDWHILE" => {
                        assert!(matches!(value, None));
                        assert!(matches!(name, None));
                        context.consume_while();
                        Box::new(expr::loops::EndWhile {
                            line: line,
                            whilestart: None,
                        })
                    },
                    "LOAD_ADDR" => {
                        assert!(matches!(value, None));
                        let name = name.unwrap().as_str().to_owned();
                        Box::new(expr::thread::LoadAddr { label: name })
                    },
                    "LOAD_CHANNEL" => {
                        assert!(matches!(name, None));
                        let value 
                            = ValueType::from_str_radix(value.unwrap().as_str(), 10)
                                .unwrap();
                        Box::new(expr::thread::LoadChannel { channel: value })
                    },
                    "SEND_CHANNEL" => {
                        assert!(matches!(value, None));
                        assert!(matches!(name, None));
                        Box::new(expr::thread::SendChannel {})
                    },
                    "RECV_CHANNEL" => {
                        assert!(matches!(value, None));
                        assert!(matches!(name, None));
                        Box::new(expr::thread::RecvChannel {})
                    },
                    "SPAWN" => {
                        assert!(matches!(value, None));
                        assert!(matches!(name, None));
                        Box::new(expr::thread::Spawn {})
                    },
                    x => panic!("Keyword '{}' is not recognized", x),
                };
            },
            None => panic!("Line not recognized by the <ParseEngine>"),
        }

        return expr;
    }
}

/*
<word>( <number>|('<str>')){0-1}
(?P<keyword>\S+)( (?P<value>\d+)|('(?P<name>\S+)')){0-1}
*/