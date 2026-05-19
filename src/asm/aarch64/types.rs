use crate::asm::error::Error;
use crate::ir;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register {
    Virtual(usize),
    Physical(u8),
    StackPointer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegSize {
    W32,
    X64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    SDiv,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cond {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operand {
    Register(Register),
    Immediate(i64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Addr {
    BaseOff { base: Register, offset: i64 },
    Global(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexOperand {
    Reg(Register),
    Imm(i64),
}

pub fn dtype_to_regsize(dtype: &ir::Dtype) -> Result<RegSize, Error> {
    match dtype {
        ir::Dtype::I1 | ir::Dtype::I32 | ir::Dtype::F32 => Ok(RegSize::W32),
        ir::Dtype::Pointer { .. } => Ok(RegSize::X64),
        _ => Err(Error::UnsupportedDtype {
            dtype: dtype.clone(),
        }),
    }
}
