use crate::interpreter::{VariableTable, StackItem, ValueType, AddrType, ControlFlow, Stack};

use super::Expr;


#[derive(Clone)]
pub struct LoadAddr {
    pub label: String
}
impl Expr for LoadAddr {
    fn name(&self) -> &'static str {
        "LoadAddr"
    }

    fn eval(&mut self,
        _thread_global: &mut VariableTable<ValueType>,
        stack: &mut Stack<StackItem>,
        _var_table: &mut VariableTable<ValueType>,
        func_table: &mut VariableTable<AddrType>,
        _control_flow: &mut ControlFlow
    ) {
        let addr = func_table.read(&self.label).unwrap();
        stack.push(StackItem::Addr(addr));
    }
}

#[derive(Clone)]
pub struct LoadChannel {
    pub channel: ValueType
}
impl Expr for LoadChannel {
    fn name(&self) -> &'static str {
        "LoadChannel"
    }

    fn eval(&mut self,
        _thread_global: &mut VariableTable<ValueType>,
        stack: &mut Stack<StackItem>,
        _var_table: &mut VariableTable<ValueType>,
        _func_table: &mut VariableTable<AddrType>,
        _control_flow: &mut ControlFlow
    ) {
        stack.push(StackItem::Channel(self.channel));
    }
}

#[derive(Clone)]
pub struct SendChannel {}
impl Expr for SendChannel {
    fn name(&self) -> &'static str {
        "SendChannel"
    }

    fn eval(&mut self,
        thread_global: &mut VariableTable<ValueType>,
        stack: &mut Stack<StackItem>,
        _var_table: &mut VariableTable<ValueType>,
        _func_table: &mut VariableTable<AddrType>,
        _control_flow: &mut ControlFlow
    ) {
        let channel = stack.pop().unwrap().channel();
        let value = stack.pop().unwrap().value();
        let channel_name = format!("_' ch{}", channel);
        thread_global.write(&channel_name, value);
    }
}

#[derive(Clone)]
pub struct RecvChannel {}
impl Expr for RecvChannel {
    fn name(&self) -> &'static str {
        "RecvChannel"
    }

    fn eval(&mut self,
        thread_global: &mut VariableTable<ValueType>,
        stack: &mut Stack<StackItem>,
        _var_table: &mut VariableTable<ValueType>,
        _func_table: &mut VariableTable<AddrType>,
        control_flow: &mut ControlFlow
    ) {
        let channel = stack.pop().unwrap().channel();
        let channel_name = format!("_' ch{}", channel);
        let recv = thread_global.read(&channel_name);
        match recv {
            Some(value) => stack.push(StackItem::Value(value)),
            None => *control_flow = ControlFlow::Block,
        };
    }
}

#[derive(Clone)]
pub struct Spawn {}
impl Expr for Spawn {
    fn name(&self) -> &'static str {
        "Spawn"
    }

    fn eval(&mut self,
        _thread_global: &mut VariableTable<ValueType>,
        stack: &mut Stack<StackItem>,
        _var_table: &mut VariableTable<ValueType>,
        _func_table: &mut VariableTable<AddrType>,
        control_flow: &mut ControlFlow
    ) {
        let f1 = stack.pop().unwrap().addr();
        let f2 = stack.pop().unwrap().addr();
        *control_flow = ControlFlow::Spawn(f1, f2);
    }
}