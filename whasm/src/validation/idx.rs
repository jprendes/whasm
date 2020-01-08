use crate::validation::{Validate, Context, Result, Error, stacks::Frame};
use crate::structure::{idx, ty};

impl<'a> Validate<'a> for idx::Type {
    type ValidationResult = &'a ty::Func;
    fn validate(&self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        let idx = self.0 as usize;
        if idx >= ctx.types.len() {
            return Err(Error::UnexpectedEndOfFile);
        }
        Ok(ctx.types[idx])
    }
}

impl<'a> Validate<'a> for idx::Func {
    type ValidationResult = &'a ty::Func;
    fn validate(&self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        let idx = self.0 as usize;
        if idx >= ctx.funcs.len() {
            return Err(Error::UnexpectedEndOfFile);
        }
        Ok(ctx.funcs[idx])
    }
}

impl<'a> Validate<'a> for idx::Table {
    type ValidationResult = &'a ty::Table;
    fn validate(&self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        let idx = self.0 as usize;
        if idx >= ctx.tables.len() {
            return Err(Error::UnexpectedEndOfFile);
        }
        Ok(ctx.tables[idx])
    }
}

impl<'a> Validate<'a> for idx::Mem {
    type ValidationResult = &'a ty::Mem;
    fn validate(&self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        let idx = self.0 as usize;
        if idx >= ctx.mems.len() {
            return Err(Error::UnexpectedEndOfFile);
        }
        Ok(ctx.mems[idx])
    }
}

impl<'a> Validate<'a> for idx::Global {
    type ValidationResult = &'a ty::Global;
    fn validate(&self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        let idx = self.0 as usize;
        if idx >= ctx.globals.len() {
            return Err(Error::UnexpectedEndOfFile);
        }
        Ok(ctx.globals[idx])
    }
}

impl<'a> Validate<'a> for idx::Local {
    type ValidationResult = &'a ty::Val;
    fn validate(&self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        let idx = self.0 as usize;
        if idx >= ctx.locals.len() {
            return Err(Error::UnexpectedEndOfFile);
        }
        Ok(ctx.locals[idx])
    }
}

impl<'a> Validate<'a> for idx::Label {
    type ValidationResult = Frame<'a>;
    fn validate(&'a self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        let idx = self.0 as usize;
        // the stack must have at least idx+1 elements
        // since the base frame can not be indexed with
        // a label.
        if !( idx + 1 < ctx.stacks.frames.len() ) {
            return Err(Error::UnexpectedEndOfFile);
        }
        let idx = ctx.stacks.frames.len() - idx - 1;
        Ok(ctx.stacks.frames[idx])
    }
}