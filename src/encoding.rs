#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Instruction {
    prefix: Vec<Prefix>,
    rex: Option<Rex>,
    opcode: Opcode,
    modrm: Option<ModRm>,
    sib: Option<Sib>,
    displacement: Option<Displacement>,
    immediate: Option<Immediate>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Prefix {
    Lock,
    Repne,
    Rep,
    Bound,
    Cs,
    Ss,
    Ds,
    Es,
    Fs,
    Gs,
    BranchNotTaken,
    BranchTaken,
    OperandSize,
    AddressSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rex {
    w: bool,
    r: bool,
    x: bool,
    b: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModRm {
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sib {
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Displacement {
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Immediate {
}
