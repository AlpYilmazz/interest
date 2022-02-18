use crate::interpreter::{Stack, VariableTable, ControlFlow, StackItem, ValueType, AddrType};

use super::Expr;


#[derive(Clone)]
pub struct Jump {
    pub label: String
}
impl Expr for Jump {
    fn name(&self) -> &'static str {
        "Jump"
    }

    fn eval(&mut self,
        _thread_global: &mut VariableTable<ValueType>,
        _stack: &mut Stack<StackItem>,
        _var_table: &mut VariableTable<ValueType>,
        func_table: &mut VariableTable<AddrType>,
        control_flow: &mut ControlFlow
    ) {
        let line = func_table.read(self.label.as_str()).unwrap();
        *control_flow = ControlFlow::JumpTo(line);
    }
}

#[derive(Clone)]
pub struct JumpZero {
    pub label: String
}
impl Expr for JumpZero {
    fn name(&self) -> &'static str {
        "JumpZero"
    }

    fn eval(&mut self,
        _thread_global: &mut VariableTable<ValueType>,
        stack: &mut Stack<StackItem>,
        _var_table: &mut VariableTable<ValueType>,
        func_table: &mut VariableTable<AddrType>,
        control_flow: &mut ControlFlow
    ) {
        let pred_opt = stack.top();
        if let Some(StackItem::Value(pred)) = pred_opt {
            if pred == 0 {
                let line = func_table.read(self.label.as_str()).unwrap();
                *control_flow = ControlFlow::JumpTo(line);
            }
        }
    }
}