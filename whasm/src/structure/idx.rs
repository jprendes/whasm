#[derive(Copy, Clone)]
#[derive(Debug, PartialEq)]
pub struct Type(pub u32);

#[derive(Debug, PartialEq)]
pub struct Func(pub u32);

#[derive(Debug, PartialEq)]
pub struct Table(pub u32);

#[derive(Debug, PartialEq)]
pub struct Mem(pub u32);

#[derive(Debug, PartialEq)]
pub struct Global(pub u32);

#[derive(Debug, PartialEq)]
pub struct Local(pub u32);

#[derive(Debug, PartialEq)]
pub struct Label(pub u32);