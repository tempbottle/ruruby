use crate::*;
use std::ops::Deref;
use std::ops::{Index, IndexMut, Range};

const ARG_ARRAY_SIZE: usize = 8;

#[derive(Debug, Clone)]
pub struct Args {
    pub block: Option<MethodRef>,
    pub kw_arg: Option<Value>,
    elems: ArgsArray,
}

impl Args {
    pub fn new(len: usize) -> Self {
        Args {
            block: None,
            kw_arg: None,
            elems: ArgsArray::new(len),
        }
    }

    pub fn push(&mut self, val: Value) {
        self.elems.push(val);
    }

    pub fn new0() -> Self {
        Args {
            block: None,
            kw_arg: None,
            elems: ArgsArray::new0(),
        }
    }

    pub fn new1(arg: Value) -> Self {
        Args {
            block: None,
            kw_arg: None,
            elems: ArgsArray::new1(arg),
        }
    }

    pub fn new2(arg0: Value, arg1: Value) -> Self {
        Args {
            block: None,
            kw_arg: None,
            elems: ArgsArray::new2(arg0, arg1),
        }
    }

    pub fn new3(
        block: impl Into<Option<MethodRef>>,
        arg0: Value,
        arg1: Value,
        arg2: Value,
    ) -> Self {
        Args {
            block: block.into(),
            kw_arg: None,
            elems: ArgsArray::new3(arg0, arg1, arg2),
        }
    }

    pub fn len(&self) -> usize {
        self.elems.len()
    }

    pub fn into_vec(self) -> Vec<Value> {
        match self.elems {
            ArgsArray::Array { ary, len } => ary[0..len].to_vec(),
            ArgsArray::Vec(v) => v,
        }
    }
}

impl Index<usize> for Args {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elems[index]
    }
}

impl Index<Range<usize>> for Args {
    type Output = [Value];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.elems[range]
    }
}

impl IndexMut<usize> for Args {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elems[index]
    }
}

impl Deref for Args {
    type Target = [Value];
    fn deref(&self) -> &Self::Target {
        self.elems.deref()
    }
}

#[derive(Debug, Clone)]
enum ArgsArray {
    Array {
        len: usize,
        ary: [Value; ARG_ARRAY_SIZE],
    },
    Vec(Vec<Value>),
}

impl ArgsArray {
    fn new(len: usize) -> Self {
        if len <= ARG_ARRAY_SIZE {
            ArgsArray::Array {
                len,
                ary: [Value::uninitialized(); ARG_ARRAY_SIZE],
            }
        } else {
            ArgsArray::Vec(vec![Value::uninitialized(); len])
        }
    }

    fn push(&mut self, val: Value) {
        match self {
            ArgsArray::Vec(ref mut v) => v.push(val),
            ArgsArray::Array {
                ref mut len,
                ref mut ary,
            } => {
                if *len == ARG_ARRAY_SIZE {
                    let mut ary = ary.to_vec();
                    ary.push(val);
                    std::mem::replace(self, ArgsArray::Vec(ary));
                } else {
                    ary[*len] = val;
                    *len += 1;
                }
            }
        }
    }

    fn new0() -> Self {
        ArgsArray::Array {
            len: 0,
            ary: [Value::uninitialized(); ARG_ARRAY_SIZE],
        }
    }

    fn new1(arg: Value) -> Self {
        let mut ary = [Value::uninitialized(); ARG_ARRAY_SIZE];
        ary[0] = arg;
        ArgsArray::Array { len: 1, ary }
    }

    fn new2(arg0: Value, arg1: Value) -> Self {
        let mut ary = [Value::uninitialized(); ARG_ARRAY_SIZE];
        ary[0] = arg0;
        ary[1] = arg1;
        ArgsArray::Array { len: 2, ary }
    }

    fn new3(arg0: Value, arg1: Value, arg2: Value) -> Self {
        let mut ary = [Value::uninitialized(); ARG_ARRAY_SIZE];
        ary[0] = arg0;
        ary[1] = arg1;
        ary[2] = arg2;
        ArgsArray::Array { len: 3, ary }
    }

    fn len(&self) -> usize {
        match self {
            ArgsArray::Array { len, .. } => *len,
            ArgsArray::Vec(v) => v.len(),
        }
    }
}

impl Index<usize> for ArgsArray {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            ArgsArray::Array { ary, .. } => &ary[index],
            ArgsArray::Vec(v) => &v[index],
        }
    }
}

impl Index<Range<usize>> for ArgsArray {
    type Output = [Value];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        match self {
            ArgsArray::Array { ary, .. } => &ary[range],
            ArgsArray::Vec(v) => &v[range],
        }
    }
}

impl IndexMut<usize> for ArgsArray {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            ArgsArray::Array { ary, .. } => &mut ary[index],
            ArgsArray::Vec(v) => &mut v[index],
        }
    }
}

impl Deref for ArgsArray {
    type Target = [Value];

    fn deref(&self) -> &Self::Target {
        match self {
            ArgsArray::Array { len, ary } => &ary[0..*len],
            ArgsArray::Vec(v) => &v,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn args() {
        let mut args = Args::new(0);
        for i in 0..20 {
            args.push(Value::fixnum(i as i64));
        }
        for i in 0..20 {
            assert_eq!(i as i64, args[i].as_fixnum().unwrap());
        }
        args[3] = Value::false_val();
        args[17] = Value::true_val();
        assert_eq!(Value::false_val(), args[3]);
        assert_eq!(Value::true_val(), args[17]);
    }

    #[test]
    fn args1() {
        let args = Args::new1(Value::fixnum(0));
        assert_eq!(0, args[0].as_fixnum().unwrap());
    }

    #[test]
    fn args2() {
        let args = Args::new2(Value::fixnum(0), Value::fixnum(1));
        assert_eq!(0, args[0].as_fixnum().unwrap());
        assert_eq!(1, args[1].as_fixnum().unwrap());
    }

    #[test]
    fn args3() {
        let args = Args::new3(None, Value::fixnum(0), Value::fixnum(1), Value::fixnum(2));
        assert_eq!(0, args[0].as_fixnum().unwrap());
        assert_eq!(1, args[1].as_fixnum().unwrap());
        assert_eq!(2, args[2].as_fixnum().unwrap());
    }
}
