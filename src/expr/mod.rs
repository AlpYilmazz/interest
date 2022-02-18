use crate::interpreter::{Stack, VariableTable, ControlFlow, StackItem, ValueType, AddrType};
use crate::bytecode::CodeContext;

pub mod flow;
pub mod loops;
pub mod thread;

pub trait Expr {
    fn name(&self) -> &'static str;
    fn init(&mut self, _context: &CodeContext) {
        println!("{} no init", self.name());
    }
    fn eval(&mut self,
        thread_global: &mut VariableTable<ValueType>,
        stack: &mut Stack<StackItem>,
        var_table: &mut VariableTable<ValueType>,
        func_table: &mut VariableTable<AddrType>,
        control_flow: &mut ControlFlow
    );
}

#[derive(Clone)]
pub struct Noop {}
impl Expr for Noop {
    fn name(&self) -> &'static str {
        "Noop"
    }
    
    fn eval(&mut self,
        _thread_global: &mut VariableTable<ValueType>,
        _stack: &mut Stack<StackItem>,
        _var_table: &mut VariableTable<ValueType>,
        _func_table: &mut VariableTable<AddrType>,
        _control_flow: &mut ControlFlow
    ) {
    }
}

#[derive(Clone)]
pub struct Start {}
impl Expr for Start {
    fn name(&self) -> &'static str {
        "Start"
    }
    
    fn eval(&mut self,
        _thread_global: &mut VariableTable<ValueType>,
        _stack: &mut Stack<StackItem>,
        _var_table: &mut VariableTable<ValueType>,
        _func_table: &mut VariableTable<AddrType>,
        _control_flow: &mut ControlFlow
    ) {
    }
}

#[derive(Clone)]
pub struct Label {}
impl Expr for Label {
    fn name(&self) -> &'static str {
        "Label"
    }

    fn eval(&mut self,
        _thread_global: &mut VariableTable<ValueType>,
        _stack: &mut Stack<StackItem>,
        _var_table: &mut VariableTable<ValueType>,
        _func_table: &mut VariableTable<AddrType>,
        _control_flow: &mut ControlFlow
    ) {
    }
}

#[derive(Clone)]
pub struct LoadVal {
    pub literal: ValueType
}
impl Expr for LoadVal {
    fn name(&self) -> &'static str {
        "LoadVal"
    }

    fn eval(&mut self,
        _thread_global: &mut VariableTable<ValueType>,
        stack: &mut Stack<StackItem>,
        _var_table: &mut VariableTable<ValueType>,
        _func_table: &mut VariableTable<AddrType>,
        _control_flow: &mut ControlFlow
    ) {
        stack.push(StackItem::Value(self.literal));
    }
}

#[derive(Clone)]
pub struct WriteVar {
    pub var_name: String
}
impl Expr for WriteVar {
    fn name(&self) -> &'static str {
        "WriteVar"
    }

    fn eval(&mut self,
        _thread_global: &mut VariableTable<ValueType>,
        stack: &mut Stack<StackItem>,
        var_table: &mut VariableTable<ValueType>,
        _func_table: &mut VariableTable<AddrType>,
        _control_flow: &mut ControlFlow
    ) {
        let val = stack.pop().unwrap().value();
        var_table.write(self.var_name.as_str(), val);
    }
}

#[derive(Clone)]
pub struct ReadVar {
    pub var_name: String
}
impl Expr for ReadVar {
    fn name(&self) -> &'static str {
        "ReadVar"
    }

    fn eval(&mut self,
        _thread_global: &mut VariableTable<ValueType>,
        stack: &mut Stack<StackItem>,
        var_table: &mut VariableTable<ValueType>,
        _func_table: &mut VariableTable<AddrType>,
        _control_flow: &mut ControlFlow
    ) {
        let val = var_table.read(self.var_name.as_str()).unwrap();
        stack.push(StackItem::Value(val));
    }
}

#[derive(Clone)]
pub struct Add {}
impl Expr for Add {
    fn name(&self) -> &'static str {
        "Add"
    }

    fn eval(&mut self,
        _thread_global: &mut VariableTable<ValueType>,
        stack: &mut Stack<StackItem>,
        _var_table: &mut VariableTable<ValueType>,
        _func_table: &mut VariableTable<AddrType>,
        _control_flow: &mut ControlFlow
    ) {
        let val1 = stack.pop().unwrap().value();
        let val2 = stack.pop().unwrap().value();
        stack.push(StackItem::Value(val1 + val2));
    }
}

#[derive(Clone)]
pub struct Multiply {}
impl Expr for Multiply {
    fn name(&self) -> &'static str {
        "Multiply"
    }

    fn eval(&mut self,
        _thread_global: &mut VariableTable<ValueType>,
        stack: &mut Stack<StackItem>,
        _var_table: &mut VariableTable<ValueType>,
        _func_table: &mut VariableTable<AddrType>,
        _control_flow: &mut ControlFlow
    ) {
        let val1 = stack.pop().unwrap().value();
        let val2 = stack.pop().unwrap().value();
        stack.push(StackItem::Value(val1 * val2));
    }
}


#[derive(Clone)]
pub struct Call {
    pub func_name: String,
    pub return_line: AddrType
}
impl Expr for Call {
    fn name(&self) -> &'static str {
        "Call"
    }

    fn eval(&mut self,
        _thread_global: &mut VariableTable<ValueType>,
        stack: &mut Stack<StackItem>,
        _var_table: &mut VariableTable<ValueType>,
        func_table: &mut VariableTable<AddrType>,
        control_flow: &mut ControlFlow
    ) {
        let line = func_table.read(self.func_name.as_str()).unwrap();
        stack.push( StackItem::ReturnAddr(self.return_line));
        *control_flow = ControlFlow::JumpTo(line);
    }
}

#[derive(Clone)]
pub struct Return {}
impl Expr for Return {
    fn name(&self) -> &'static str {
        "Return"
    }

    fn eval(&mut self,
        _thread_global: &mut VariableTable<ValueType>,
        stack: &mut Stack<StackItem>,
        _var_table: &mut VariableTable<ValueType>,
        _func_table: &mut VariableTable<AddrType>,
        control_flow: &mut ControlFlow
    ) {
        loop {
            let s_top = stack.pop().unwrap();
            if let StackItem::ReturnAddr(return_line) = s_top {
                *control_flow = ControlFlow::JumpTo(return_line);
                return;
            }
        }
    }
}

#[derive(Clone)]
pub struct ReturnValue {}
impl Expr for ReturnValue {
    fn name(&self) -> &'static str {
        "ReturnValue"
    }

    fn eval(&mut self,
        _thread_global: &mut VariableTable<ValueType>,
        stack: &mut Stack<StackItem>,
        _var_table: &mut VariableTable<ValueType>,
        _func_table: &mut VariableTable<AddrType>,
        control_flow: &mut ControlFlow
    ) {
        let return_val = stack.pop().unwrap();
        loop {
            let s_top = stack.pop().unwrap();
            if let StackItem::ReturnAddr(return_line) = s_top {
                stack.push(return_val);
                *control_flow = ControlFlow::JumpTo(return_line);
                return;
            }
        }
    }
}