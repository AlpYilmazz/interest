use std::{fs::File, io::Read};

use crate::expr;
use crate::interpreter::{AddrType, Stack, VariableTable};
use crate::parse::ParseEngine;


pub struct CodeContext {
    line: AddrType,
    start: AddrType,
    labels: VariableTable<AddrType>,
    pub loops: Vec<(AddrType, AddrType)>,
    pub whiles: Vec<(AddrType, AddrType)>,
    loop_stack: Stack<AddrType>,
    while_stack: Stack<AddrType>,
}

impl CodeContext {
    pub fn set_line(&mut self, line: AddrType) {
        self.line = line;
    }

    pub fn line(&self) -> AddrType {
        self.line
    }

    pub fn start(&self) -> AddrType {
        self.start
    }

    pub fn set_start(&mut self, start: AddrType) {
        self.start = start;
    }

    pub fn set_label(&mut self, label: &str, line: AddrType) {
        self.labels.write_unique(label, line);
    }

    pub fn push_loop(&mut self) {
        self.loop_stack.push(self.line);
    }

    pub fn consume_loop(&mut self) {
        let loopstart = self.loop_stack.pop().unwrap();
        let endloop = self.line;
        self.loops.push((loopstart, endloop));
    }

    pub fn push_while(&mut self) {
        self.while_stack.push(self.line);
    }

    pub fn consume_while(&mut self) {
        let whilestart = self.while_stack.pop().unwrap();
        let endwhile = self.line;
        self.whiles.push((whilestart, endwhile));
    }
}

impl Default for CodeContext {
    fn default() -> Self {
        Self {
            line: Default::default(),
            start: Default::default(),
            labels: Default::default(),
            loops: Vec::new(),
            whiles: Vec::new(),
            loop_stack: Stack::new(),
            while_stack: Stack::new(),
        }
    }
}

pub struct ByteCode {
    labels: VariableTable<AddrType>,
    exprs: Vec<Box<dyn expr::Expr>>,
    start_addr: AddrType,
}

impl ByteCode {
    pub fn from(path: &str) -> Self {
        let mut file_content: String = String::new();
        File::open(path).unwrap().read_to_string(&mut file_content).unwrap();

        let mut lines: Vec<String> = file_content.lines()
                .filter(|line| !line.trim().is_empty())
                .map(|s| s.trim().to_owned())
                .collect();

        let parse_engine: ParseEngine = Default::default();
        let mut code_context: CodeContext = Default::default();
        let mut ctx = &mut code_context;
        let mut exprs: Vec<Box<dyn expr::Expr>> = lines.drain(..)
                .enumerate()
                .map(|(i, s)| {
                    ctx.set_line(i);
                    parse_engine.parse(&s, &mut ctx)
                })
                .collect();

        for expr in &mut exprs {
            expr.init(ctx);
        }

        let start_addr = code_context.start();
        let labels = code_context.labels;
        
        ByteCode {
            //source: source,
            //expressions: expr_iter
            labels: labels,
            exprs: exprs,
            start_addr: start_addr,
        }
    }

    pub fn get_labels(&self) -> &VariableTable<AddrType> {
        &self.labels
    }

    pub fn start_addr(&self) -> AddrType {
        self.start_addr
    }

    pub fn end_addr(&self) -> AddrType {
        self.exprs.len()
    }

    pub fn get_line<'a>(&'a self, addr: usize) -> Option<&'a Box<dyn expr::Expr>> {
        self.exprs.get(addr)
    }

    pub fn get_line_mut<'a>(&'a mut self, addr: usize) -> Option<&'a mut Box<dyn expr::Expr>> {
        self.exprs.get_mut(addr)
    }
}