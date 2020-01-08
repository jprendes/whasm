use crate::validation::{Validate, Context, Result, Error};
use crate::structure::ty;

impl<'a> Validate<'a> for ty::Limits {
    type ValidationResult = u32;
    fn validate(&self, _ctx: &mut Context) -> Result<Self::ValidationResult> {
        let min = self.min;
        let max = self.max.unwrap_or(self.min);
        if !(min <= max) {
            return Err(Error::UnexpectedEndOfFile);
        }
        Ok(max)
    }
}

impl<'a> Validate<'a> for ty::Func {
    type ValidationResult = ();
    fn validate(&self, _ctx: &mut Context) -> Result<Self::ValidationResult> {
        if !(self.results.len() <= 1) {
            return Err(Error::UnexpectedEndOfFile);
        }
        Ok(())
    }
}

impl<'a> Validate<'a> for ty::Table {
    type ValidationResult = ();
    fn validate(&'a self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        let range = self.lim.validate(ctx)?;
        if !(range <= std::u32::MAX) {
            return Err(Error::UnexpectedEndOfFile);
        }
        Ok(())
    }
}

impl<'a> Validate<'a> for ty::Mem {
    type ValidationResult = ();
    fn validate(&'a self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        let range = self.lim.validate(ctx)?;
        if !(range <= (std::u16::MAX as u32)) {
            return Err(Error::UnexpectedEndOfFile);
        }
        Ok(())
    }
}

impl<'a> Validate<'a> for ty::Global {
    type ValidationResult = ();
    fn validate(&self, _ctx: &mut Context) -> Result<Self::ValidationResult> {
        Ok(())
    }
}

impl<'a> Validate<'a> for ty::RetVal {
    type ValidationResult = &'a [ty::Val];
    fn validate(&'a self, _ctx: &mut Context) -> Result<Self::ValidationResult> {
        Ok(&self.0[..])
    }
}
