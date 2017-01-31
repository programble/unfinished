// TODO: Type bounds.

pub struct Lock<I>(pub I);

pub struct Repne<I>(pub I);
pub type Repnz<I> = Repne<I>;

pub struct Rep<I>(pub I);
pub type Repe<I> = Rep<I>;
pub type Repz<I> = Rep<I>;
