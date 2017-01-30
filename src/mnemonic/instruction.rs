use mnemonic::operand::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Adc {
    AlImm8(Imm8),
    AxImm16(Imm16),
    EaxImm32(Imm32),
    RaxImm32(Imm32),

    Rm8Imm8(Rm8, Imm8),
    Rm16Imm16(Rm16, Imm16),
    Rm32Imm32(Rm32, Imm32),
    Rm64Imm32(Rm64, Imm32),

    Rm16Imm8(Rm16, Imm8),
    Rm32Imm8(Rm32, Imm8),
    Rm64Imm8(Rm64, Imm8),

    Rm8R8(Rm8, R8),
    Rm16R16(Rm16, R16),
    Rm32R32(Rm32, R32),
    Rm64R64(Rm64, R64),

    R8Rm8(R8, Rm8),
    R16Rm16(R16, Rm16),
    R32Rm32(R32, Rm32),
    R64Rm64(R64, Rm64),
}
