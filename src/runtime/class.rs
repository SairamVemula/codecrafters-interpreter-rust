use crate::{
    ast::expr::Object,
    runtime::{Callable, Function, Instance, Result},
};
use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub methods: HashMap<String, Function>,
    pub superclass: Option<Box<Class>>,
}

impl Class {
    pub fn new(
        name: String,
        methods: HashMap<String, Function>,
        superclass: Option<Class>,
    ) -> Self {
        Self {
            name,
            methods,
            superclass: superclass.map(Box::new),
        }
    }
    pub fn find_method(&self, name: &str) -> Option<Function> {
        match self.methods.get(name).cloned() {
            Some(method) => Some(method),
            None => {
                if let Some(superclass) = &self.superclass {
                    superclass.find_method(name)
                } else {
                    None
                }
            }
        }
    }
}

impl Callable for Class {
    fn call(&self, interpreter: &mut super::Interpreter, args: Vec<Object>) -> Result<Object> {
        let instance = Rc::new(RefCell::new(Instance::new(self.clone())));
        let initializer = self.find_method("init");
        if let Some(initializer) = initializer {
            initializer.bind(instance.clone()).call(interpreter, args)?;
        }
        Ok(Object::Instance(instance))
    }

    fn arity(&self) -> usize {
        let initializer = self.find_method("init");
        if let Some(initializer) = initializer {
            return initializer.arity();
        }
        0
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
