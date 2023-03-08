use std::collections::HashMap;

const MAX_INSTRUCTIONS: usize = 128;

#[derive(Clone)]
pub enum Opcode {
    St,
    Ld,
    And,
    Or,
    Xor,
    Rol,
    Ror,
    Add,
    Adc,
    Neg,
    Inc,
    Dec,
    Setc,
    Clrc,
    Trfnc,
    Bz0,
    Bz1,
    Bc0,
    Bc1,
    Bv0,
    Bv1,
    Bn0,
    Bn1,
    Bra,
    Nop,
}

#[derive(Clone)]
pub enum ValueType {
    Raw(u8),
    Const(String),
    Label(String),
}

#[derive(Clone)]
pub struct Value {
    pub direct: bool,
    pub value_type: ValueType,
}

impl Value {
    pub const fn new(direct: bool, value_type: ValueType) -> Self {
        Self { direct, value_type }
    }
}

#[derive(Clone)]
pub enum Parameter {
    Acc,
    Value(Value),
}

#[derive(Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub param: Vec<Parameter>,
}

impl Instruction {
    pub fn new(opcode: Opcode, param: Vec<Parameter>) -> Self {
        Self { opcode, param }
    }
}

pub struct SyntaxTree {
    instructions: Vec<Instruction>,
    constants: HashMap<String, u8>,
    labels: HashMap<String, u8>,
}

impl SyntaxTree {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: HashMap::new(),
            labels: HashMap::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) -> Result<(), String> {
        for parameter in &instruction.param {
            if let Parameter::Value(value) = parameter {
                if let ValueType::Label(_) = value.value_type {
                    if !value.direct {
                        return Err("Indirect addressing on labels is not allowed".to_owned());
                    }
                }
            }
        }

        self.instructions.push(instruction);

        if self.instructions.len() > MAX_INSTRUCTIONS {
            Err(format!("Too many instructions, a program can contain at most {MAX_INSTRUCTIONS} instructions"))
        } else {
            Ok(())
        }
    }

    pub fn add_const(&mut self, const_name: &str, global_value: u8) -> Result<(), String> {
        if self
            .constants
            .insert(const_name.to_owned(), global_value)
            .is_some()
        {
            Err(format!("A global named {const_name} already exists"))
        } else {
            Ok(())
        }
    }

    pub fn add_label(&mut self, label_name: &str) -> Result<(), String> {
        if self
            .labels
            .insert(label_name.to_owned(), self.instructions.len() as u8)
            .is_some()
        {
            Err(format!("A label named {label_name} already exists"))
        } else {
            Ok(())
        }
    }

    // Replace constant and label name by their raw value
    fn process_instruction(&self, instruction: &Instruction) -> Result<Instruction, String> {
        let mut new_parameters = Vec::new();

        for parameter in &instruction.param {
            new_parameters.push(if let Parameter::Value(value) = parameter {
                match &value.value_type {
                    ValueType::Const(const_name) => {
                        if let Some(const_value) = self.constants.get(const_name) {
                            Parameter::Value(Value::new(
                                value.direct,
                                ValueType::Raw(const_value.to_owned()),
                            ))
                        } else {
                            return Err(format!("Constant named {const_name} doesn't exist"));
                        }
                    }
                    ValueType::Label(label_name) => {
                        if let Some(label_value) = self.labels.get(label_name) {
                            Parameter::Value(Value::new(
                                value.direct,
                                ValueType::Raw(label_value.to_owned()),
                            ))
                        } else {
                            return Err(format!("Label named {label_name} doesn't exist"));
                        }
                    }
                    _ => parameter.clone(),
                }
            } else {
                parameter.clone()
            });
        }

        Ok(Instruction::new(instruction.opcode.clone(), new_parameters))
    }

    /// Generate the list of instructions, ready to be converted to machine code
    /// Replaces constants and labels with their raw value
    pub fn generate_instructions(&self) -> Result<Vec<Instruction>, String> {
        let mut checked_instructions = Vec::new();

        for instruction in &self.instructions {
            match self.process_instruction(instruction) {
                Ok(instr) => {
                    checked_instructions.push(instr);
                }
                Err(errmsg) => {
                    return Err(errmsg);
                }
            }
        }

        Ok(checked_instructions)
    }
}
