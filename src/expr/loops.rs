use crate::interpreter::{Stack, VariableTable, ControlFlow, StackItem, AddrType, ValueType};
use crate::bytecode::CodeContext;

use super::Expr;

#[derive(Clone, Debug)]
pub struct Loop {
    // Loop 0..{stack.top() (= Some(Value(end)))}
    pub line: AddrType,
    pub loop_var: Option<String>,
    pub count: Option<ValueType>,
    pub endloop: Option<AddrType>
}
impl Expr for Loop {
    fn name(&self) -> &'static str {
        "Loop"
    }

    fn init(&mut self, context: &CodeContext) {
        let loops = &context.loops;
        for (loopstart, endloop) in loops {
            if *loopstart == self.line {
                self.loop_var = Some(format!("_' i{}", *loopstart));
                self.endloop = Some(*endloop);
                break;
            }
        }
    }

    fn eval(&mut self,
        _thread_global: &mut VariableTable<ValueType>,
        stack: &mut Stack<StackItem>,
        var_table: &mut VariableTable<ValueType>,
        _func_table: &mut VariableTable<AddrType>,
        control_flow: &mut ControlFlow
    ) {
        let loop_var = self.loop_var.as_ref().unwrap();
        let var = var_table.read(loop_var);
        match var {
            Some(var) => {
                if var >= self.count.unwrap() {
                    var_table.remove(loop_var);
                    self.count = None;
                    *control_flow = ControlFlow::JumpTo(self.endloop.unwrap() + 1);
                }
            },
            None => {
                self.count = Some(stack.pop().unwrap().value());
                var_table.write(loop_var, 0);
            }
        };
    }
}

#[derive(Clone)]
pub struct EndLoop {
    pub line: AddrType,
    pub loop_var: Option<String>,
    pub loopstart: Option<AddrType>,
}
impl Expr for EndLoop {
    fn name(&self) -> &'static str {
        "EndLoop"
    }

    fn init(&mut self, context: &CodeContext) {
        let loops = &context.loops;
        for (loopstart, endloop) in loops {
            if *endloop == self.line {
                self.loop_var = Some(format!("_' i{}", *loopstart));
                self.loopstart = Some(*loopstart);
                println!("EndLoop init: loopstart: {:?}", self.loopstart);
                break;
            }
        }
    }

    fn eval(&mut self,
        _thread_global: &mut VariableTable<ValueType>,
        _stack: &mut Stack<StackItem>,
        var_table: &mut VariableTable<ValueType>,
        _func_table: &mut VariableTable<AddrType>,
        control_flow: &mut ControlFlow
    ) {
        let mut loop_var_val = var_table.read(self.loop_var.as_ref().unwrap()).unwrap();
        loop_var_val += 1;
        println!("loop_var_val: {} (after inc)", loop_var_val);
        var_table.write(self.loop_var.as_ref().unwrap(), loop_var_val);
        
        *control_flow = ControlFlow::JumpTo(self.loopstart.unwrap());
    }
}

#[derive(Clone, Debug)]
pub struct While {
    // While stack.top() != Some(Value(0))
    pub line: AddrType,
    pub endwhile: Option<AddrType>,
}
impl Expr for While {
    fn name(&self) -> &'static str {
        "While"
    }

    fn init(&mut self, context: &CodeContext) {
        let whiles = &context.whiles;
        for (whilestart, endwhile) in whiles {
            if *whilestart == self.line {
                self.endwhile = Some(*endwhile);
                break;
            }
        }
    }

    fn eval(&mut self,
        _thread_global: &mut VariableTable<ValueType>,
        stack: &mut Stack<StackItem>,
        _var_table: &mut VariableTable<ValueType>,
        _func_table: &mut VariableTable<AddrType>,
        control_flow: &mut ControlFlow
    ) {
        let s_top = stack.top();
        match s_top {
            Some(StackItem::Value(val)) if val != 0 => {
                // continue
            },
            _ => { // Some(Value(0)) | Some(! Value) | None
                // break
                *control_flow = ControlFlow::JumpTo(self.endwhile.unwrap() + 1);
            },
        }
    }
}

#[derive(Clone)]
pub struct EndWhile {
    pub line: AddrType,
    pub whilestart: Option<AddrType>,
}
impl Expr for EndWhile {
    fn name(&self) -> &'static str {
        "EndWhile"
    }

    fn init(&mut self, context: &CodeContext) {
        let whiles = &context.whiles;
        for (whilestart, endwhile) in whiles {
            if *endwhile == self.line {
                self.whilestart = Some(*whilestart);
                println!("EndWhile init: whilestart: {:?}", self.whilestart);
                break;
            }
        }
    }

    fn eval(&mut self,
        _thread_global: &mut VariableTable<ValueType>,
        _stack: &mut Stack<StackItem>,
        _var_table: &mut VariableTable<ValueType>,
        _func_table: &mut VariableTable<AddrType>,
        control_flow: &mut ControlFlow
    ) {
        *control_flow = ControlFlow::JumpTo(self.whilestart.unwrap());
    }
}