use crate::*;

#[derive(Debug, Clone)]
pub struct ArrayInfo {
    pub elements: Vec<Value>,
}

impl ArrayInfo {
    pub fn new(elements: Vec<Value>) -> Self {
        ArrayInfo { elements }
    }

    pub fn get_elem(&self, vm: &mut VM, args: &Args) -> VMResult {
        let arg_num = args.len();
        vm.check_args_range(arg_num, 1, 2)?;
        let index = args[0].expect_integer(&vm, "Index")?;
        let index = vm.get_array_index(index, self.elements.len())?;
        let val = if arg_num == 1 {
            if index >= self.elements.len() {
                Value::nil()
            } else {
                self.elements[index]
            }
        } else {
            let len = args[1].expect_integer(&vm, "Index")?;
            if len < 0 {
                Value::nil()
            } else if index >= self.elements.len() {
                Value::array_from(&vm.globals, vec![])
            } else {
                let len = len as usize;
                let end = std::cmp::min(self.elements.len(), index + len);
                let ary = (&self.elements[index..end]).to_vec();
                Value::array_from(&vm.globals, ary)
            }
        };
        Ok(val)
    }

    pub fn set_elem(&mut self, vm: &mut VM, args: &Args) -> VMResult {
        vm.check_args_range(args.len(), 2, 3)?;
        let val = if args.len() == 3 { args[2] } else { args[1] };
        let index = args[0].expect_integer(&vm, "Index")?;
        let elements = &mut self.elements;
        let len = elements.len();
        if args.len() == 2 {
            if index >= elements.len() as i64 {
                let padding = index as usize - len;
                elements.append(&mut vec![Value::nil(); padding]);
                elements.push(val);
            } else {
                let index = vm.get_array_index(index, len)?;
                elements[index] = val;
            }
        } else {
            let index = vm.get_array_index(index, len)?;
            let length = args[1].expect_integer(&vm, "Length")?;
            if length < 0 {
                return Err(vm.error_index(format!("Negative length. {}", length)));
            };
            let length = length as usize;
            let end = std::cmp::min(len, index + length);
            match val.as_array() {
                Some(mut val) => {
                    let mut tail = elements.split_off(end);
                    elements.truncate(index);
                    elements.append(&mut val.elements.clone());
                    elements.append(&mut tail);
                }
                None => {
                    elements.drain(index..end);
                    elements.insert(index, val);
                }
            };
        };
        Ok(val)
    }

    pub fn to_s(&self, vm: &mut VM) -> String {
        match self.elements.len() {
            0 => "[]".to_string(),
            1 => format!("[{}]", vm.val_inspect(self.elements[0])),
            len => {
                let mut result = vm.val_inspect(self.elements[0]);
                for i in 1..len {
                    result = format!("{}, {}", result, vm.val_inspect(self.elements[i]));
                }
                format! {"[{}]", result}
            }
        }
    }
}

pub type ArrayRef = Ref<ArrayInfo>;

impl ArrayRef {
    pub fn from(elements: Vec<Value>) -> Self {
        ArrayRef::new(ArrayInfo::new(elements))
    }
}
