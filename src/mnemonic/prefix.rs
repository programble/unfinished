// TODO: Type bounds.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lock<I>(pub I);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Repne<I>(pub I);
pub type Repnz<I> = Repne<I>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rep<I>(pub I);
pub type Repe<I> = Rep<I>;
pub type Repz<I> = Rep<I>;
