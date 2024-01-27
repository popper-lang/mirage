use crate::{label::Label, stringify::Stringify, MirageTypeEnum, MirageValueEnum, RegisterType, RegisterValue};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
    args: Vec<MirageTypeEnum>,
    ret: MirageTypeEnum,
}

impl FunctionType {
    pub fn new(args: Vec<MirageTypeEnum>, ret: MirageTypeEnum) -> Self {
        Self { ret, args }
    }

    pub fn get_ret(&self) -> &MirageTypeEnum {
        &self.ret
    }

    pub fn get_args(&self) -> &Vec<MirageTypeEnum> {
        &self.args
    }

    pub fn fn_value(&self, name: String) -> FunctionValue {
        FunctionValue::new(name, self.clone())
    }

    pub fn print_to_string(&self) -> String {
        let mut s = String::new();
        s.push_str("(");
        for arg in &self.args {
            s.push_str(&arg.print_to_string());
            s.push_str(", ");
        }
        s.pop();
        s.pop();
        s.push_str(") -> ");
        s.push_str(&self.ret.print_to_string());
        s
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionValue {
    name: String,
    ty: FunctionType,
    labels: Vec<Label>
}

impl FunctionValue {
    pub fn new(name: String, ty: FunctionType) -> Self {
        Self { name, ty, labels: Vec::new() }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_type(&self) -> &FunctionType {
        &self.ty
    }

    pub fn get_nth_arg(&self, n: usize) -> Option<MirageValueEnum> {
        if n >= self.ty.args.len() {
            return None;
        }

        Some(MirageValueEnum::Register(RegisterValue::new(
            n,
            RegisterType::Argument,
            self.ty.args[n].clone(),
        )))
    }

    pub fn add_label(&mut self, label: Label) {
        self.labels.push(label);
    }

    pub fn print_to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.name);
        s.push_str(&self.ty.print_to_string());
        s.push_str("\n");
        s.push_str(
            &self
                .labels
                .iter()
                .map(|l| l.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
        );
        s
    }


}
