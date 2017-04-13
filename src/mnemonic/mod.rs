pub mod dsl;
pub mod instruction;
pub mod operand;

// TODO: Move.
mod nop {
    use mnemonic::dsl::*;
    pub const NOP: [Nop; 9] = [
        Nop::Eax,
        Nop::Ax,
        Nop::Rm32(M32(Offset64(None, Base(Rax)))),
        Nop::Rm32(M32(Offset64(None, BaseDisp(Rax, Disp8(0x00))))),
        Nop::Rm32(M32(Offset64(None, BaseIndexDisp(Rax, Index64::Rax, X1, Disp8(0x00))))),
        Nop::Rm16(M16(Offset64(None, BaseIndexDisp(Rax, Index64::Rax, X1, Disp8(0x00))))),
        Nop::Rm32(M32(Offset64(None, BaseDisp(Rax, Disp32(0x00000000))))),
        Nop::Rm32(M32(Offset64(None, BaseIndexDisp(Rax, Index64::Rax, X1, Disp32(0x00000000))))),
        Nop::Rm16(M16(Offset64(None, BaseIndexDisp(Rax, Index64::Rax, X1, Disp32(0x00000000))))),
    ];
}
pub use self::nop::NOP;

mod display;
