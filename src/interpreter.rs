use std::collections::HashMap;
use std::fmt::Debug;

use crate::bytecode::ByteCode;


#[derive(Default)]
pub struct Stack<T> {
    items: Vec<T>
}

impl<T: Clone + Debug> Stack<T> {
    pub fn new() -> Self {
        Stack {
            items: vec![]
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
        self.print_stack();
    }

    pub fn pop(&mut self) -> Option<T> {
        let ret = self.items.pop();
        self.print_stack();
        ret
    }

    pub fn top(&self) -> Option<T> {
        self.print_stack();
        let item = self.items.last();
        item.map_or(None, |v| Some(v.to_owned()))
    }

    pub fn print_stack(&self) {
        println!("{:?}", self.items);
    }
}


#[derive(Default, Clone)]
pub struct VariableTable<T: Clone> {
    vars: HashMap<String, T>
}

impl<T: Copy + Clone> VariableTable<T> {
    pub fn write(&mut self, name: &str, val: T) {
        self.vars.insert(String::from(name), val);
    }

    pub fn write_unique(&mut self, name: &str, val: T) {
        let unique = self.vars.insert(String::from(name), val).is_none();
        if !unique {
            panic!("Name not unique");
        }
    }

    pub fn read(&self, name: &str) -> Option<T> {
        let val = self.vars.get(name);
        val.map_or(None, |v| Some(v.to_owned()))
    }

    pub fn remove(&mut self, name: &str) {
        self.vars.remove(name);
    }
}

pub enum ControlFlow {
    Normal,
    Block,
    JumpTo(usize),
    Spawn(AddrType, AddrType)
}

pub type ValueType = i64;
pub type AddrType = usize;

#[derive(Copy, Clone, Debug)]
pub enum StackItem {
    Value(ValueType),
    Addr(AddrType),
    ReturnAddr(AddrType),
    Channel(ValueType),
}

impl StackItem {
    pub fn value(&self) -> ValueType {
        if let Self::Value(v) = self {
            return v.to_owned();
        }
        panic!("StackItem is not Value");
    }

    pub fn addr(&self) -> AddrType {
        if let Self::Addr(a) = self {
            return a.to_owned();
        }
        panic!("StackItem is not Addr");
    }

    pub fn return_addr(&self) -> AddrType {
        if let Self::ReturnAddr(a) = self {
            return a.to_owned();
        }
        panic!("StackItem is not ReturnAddr");
    }

    pub fn channel(&self) -> ValueType {
        if let Self::Channel(a) = self {
            return *a;
        }
        panic!("StackItem is not Channel");
    }
}

#[allow(dead_code)]
struct IThread {
    id: usize,
    stack: Stack<StackItem>,
    var_table: VariableTable<ValueType>,
    addr: AddrType
}

impl IThread {
    fn new(id: usize, addr: AddrType) -> Self {
        IThread {
            id: id,
            stack: Stack::<StackItem>::new(),
            var_table: Default::default(),
            addr: addr,
        }
    }

    #[allow(dead_code)]
    pub fn get_id(&self) -> usize {
        self.id
    }
}

pub struct Interpreter {
    byte_code: ByteCode,
    func_table: VariableTable<AddrType>,
    main_thread_id: usize,
    next_thread_id: usize,
    threads: HashMap<usize, IThread>,
    thread_global: VariableTable<ValueType>,
}

impl Interpreter {
    pub fn new(byte_code: ByteCode) -> Self {
        let main_thread_id = 0;
        let mut labels = byte_code.get_labels().clone();
        labels.write("_' end", byte_code.end_addr());

        let mut inter = Interpreter {
            byte_code: byte_code,
            func_table: labels,
            main_thread_id: main_thread_id,
            next_thread_id: main_thread_id + 1,
            threads: Default::default(),
            thread_global: Default::default(),
        };

        inter.threads.insert(inter.main_thread_id,
            IThread::new(inter.main_thread_id, inter.byte_code.start_addr()));

        inter
    }

    pub fn run(&mut self) {
        //let main_thread = self.threads.get_mut(&self.main_thread_id).unwrap();

        let byte_code = &mut self.byte_code;
        let mut control_flow = ControlFlow::Normal;
        let mut i = 0;

        let threads = &mut self.threads;
        let mut thread_id = self.main_thread_id;
        loop {
            i += 1;
            if i == 1000 {
                break;
            }
            
            // NEXT THREAD TO RUN
            let mut thread;
            loop {
                let thread_opt = threads.get_mut(&thread_id);
                
                thread_id += 1;
                if thread_id >= self.next_thread_id {
                    thread_id = self.main_thread_id;
                }

                if let Some(th) = thread_opt {
                    thread = th;
                    break;
                }
            }
            
            // RUN ONE INSTRUCTION FROM THE THREAD
            let mut end_thread = false;
            {
                let expr_next = byte_code.get_line_mut(thread.addr);
                match expr_next {
                    Some(expr) => {
                        println!("{}", expr.name());
                        expr.eval(&mut self.thread_global,
                            &mut thread.stack, &mut thread.var_table,
                            &mut self.func_table, &mut control_flow);

                    },
                    None => {
                        end_thread = true;
                    },
                }
            }

            // CONTROL FLOW THE THREAD
            match control_flow {
                ControlFlow::Normal => thread.addr += 1,
                ControlFlow::Block => {},
                ControlFlow::JumpTo(line) => thread.addr = line,
                ControlFlow::Spawn(f1, f2) => {
                    thread.addr += 1;

                    let end_addr = self.func_table.read("_' end").unwrap();

                    let mut id = self.next_thread_id;
                    self.next_thread_id += 1;
                    let mut th1 = IThread::new(id, f1);
                    th1.stack.push(StackItem::ReturnAddr(end_addr));
                    threads.insert(id, th1);

                    id = self.next_thread_id;
                    self.next_thread_id += 1;
                    let mut th2 = IThread::new(id, f2);
                    th2.stack.push(StackItem::ReturnAddr(end_addr));
                    threads.insert(id, th2);
                },
            };
            control_flow = ControlFlow::Normal;

            if end_thread {
                threads.remove(&thread_id);
            }

            // NEXT ITER (PROBABLY WITH THE NEXT THREAD)
        }
    }
}