use std::collections::HashMap;
//#[macro_use]
use crate::*;

/// Heap-allocated objects.
#[derive(Debug, Clone, PartialEq)]
pub struct RValue {
    class: Value,
    var_table: Box<ValueTable>,
    pub kind: ObjKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjKind {
    Ordinary,
    FixNum(i64),
    FloatNum(f64),
    Class(ClassRef),
    Module(ClassRef),
    String(RString),
    Array(ArrayRef),
    Range(RangeInfo),
    Splat(Value), // internal use only.
    Hash(HashRef),
    Proc(ProcRef),
    Regexp(RegexpRef),
    Method(MethodObjRef),
}

impl RValue {
    pub fn id(&self) -> u64 {
        self as *const RValue as u64
    }

    pub fn as_ref(&self) -> ObjectRef {
        Ref::from_ref(self)
    }

    pub fn new_bootstrap(classref: ClassRef) -> Self {
        RValue {
            class: Value::nil(), // dummy for boot strapping
            kind: ObjKind::Class(classref),
            var_table: Box::new(HashMap::new()),
        }
    }

    pub fn new_fixnum(i: i64) -> Self {
        RValue {
            class: Value::nil(),
            var_table: Box::new(HashMap::new()),
            kind: ObjKind::FixNum(i),
        }
    }

    pub fn new_flonum(f: f64) -> Self {
        RValue {
            class: Value::nil(),
            var_table: Box::new(HashMap::new()),
            kind: ObjKind::FloatNum(f),
        }
    }

    pub fn new_string(globals: &Globals, s: String) -> Self {
        RValue {
            class: globals.builtins.string,
            var_table: Box::new(HashMap::new()),
            kind: ObjKind::String(RString::Str(s)),
        }
    }

    pub fn new_bytes(globals: &Globals, b: Vec<u8>) -> Self {
        RValue {
            class: globals.builtins.string,
            var_table: Box::new(HashMap::new()),
            kind: ObjKind::String(RString::Bytes(b)),
        }
    }

    pub fn new_ordinary(class: Value) -> Self {
        RValue {
            class,
            var_table: Box::new(HashMap::new()),
            kind: ObjKind::Ordinary,
        }
    }

    pub fn new_class(globals: &Globals, classref: ClassRef) -> Self {
        RValue {
            class: globals.builtins.class,
            var_table: Box::new(HashMap::new()),
            kind: ObjKind::Class(classref),
        }
    }

    pub fn new_module(globals: &Globals, classref: ClassRef) -> Self {
        RValue {
            class: globals.builtins.module,
            var_table: Box::new(HashMap::new()),
            kind: ObjKind::Module(classref),
        }
    }

    pub fn new_array(globals: &Globals, arrayref: ArrayRef) -> Self {
        RValue {
            class: globals.builtins.array,
            var_table: Box::new(HashMap::new()),
            kind: ObjKind::Array(arrayref),
        }
    }

    pub fn new_range(globals: &Globals, range: RangeInfo) -> Self {
        RValue {
            class: globals.builtins.range,
            var_table: Box::new(HashMap::new()),
            kind: ObjKind::Range(range),
        }
    }

    pub fn new_splat(globals: &Globals, val: Value) -> Self {
        RValue {
            class: globals.builtins.array,
            var_table: Box::new(HashMap::new()),
            kind: ObjKind::Splat(val),
        }
    }

    pub fn new_hash(globals: &Globals, hashref: HashRef) -> Self {
        RValue {
            class: globals.builtins.hash,
            var_table: Box::new(HashMap::new()),
            kind: ObjKind::Hash(hashref),
        }
    }

    pub fn new_regexp(globals: &Globals, regexpref: RegexpRef) -> Self {
        RValue {
            class: globals.builtins.regexp,
            var_table: Box::new(HashMap::new()),
            kind: ObjKind::Regexp(regexpref),
        }
    }

    pub fn new_proc(globals: &Globals, procref: ProcRef) -> Self {
        RValue {
            class: globals.builtins.procobj,
            var_table: Box::new(HashMap::new()),
            kind: ObjKind::Proc(procref),
        }
    }

    pub fn new_method(globals: &Globals, methodref: MethodObjRef) -> Self {
        RValue {
            class: globals.builtins.method,
            var_table: Box::new(HashMap::new()),
            kind: ObjKind::Method(methodref),
        }
    }
}

pub type ObjectRef = Ref<RValue>;

impl RValue {
    /// Pack `self` into `Value`(64-bit data representation).
    /// This method consumes `self` and allocates it on the heap, returning `Value`,
    /// a wrapped raw pointer.  
    pub fn pack(self) -> Value {
        Value::from(Box::into_raw(Box::new(self)) as u64)
    }

    /// Return a class of the object. If the objetct has a sigleton class, return the singleton class.
    pub fn class(&self) -> Value {
        self.class
    }

    /// Return a "real" class of the object.
    pub fn search_class(&self) -> Value {
        let mut class = self.class;
        loop {
            if class.as_class().is_singleton {
                class = class.as_object().class;
            } else {
                return class;
            }
        }
    }

    /// Set a class of the object.
    pub fn set_class(&mut self, class: Value) {
        self.class = class;
    }

    pub fn get_var(&self, id: IdentId) -> Option<Value> {
        self.var_table.get(&id).cloned()
    }

    pub fn get_mut_var(&mut self, id: IdentId) -> Option<&mut Value> {
        self.var_table.get_mut(&id)
    }

    pub fn set_var(&mut self, id: IdentId, val: Value) {
        self.var_table.insert(id, val);
    }

    pub fn var_table(&mut self) -> &mut ValueTable {
        &mut self.var_table
    }

    pub fn get_instance_method(&self, id: IdentId) -> Option<MethodRef> {
        self.search_class()
            .as_class()
            .method_table
            .get(&id)
            .cloned()
    }
}