use crate::validation::{Validate, ValidationEntry, Context, Result, Error};
use crate::structure::module;
use crate::structure::ty;

pub mod result {
    use crate::structure::ty;

    pub struct Module<'a> {
        pub imports: Vec<External<'a>>,
        pub exports: Vec<External<'a>>,
    }

    pub enum External<'a> {
        Func ( &'a ty::Func ),
        Table ( &'a ty::Table ),
        Mem ( &'a ty::Mem ),
        Global ( &'a ty::Global ),
    }
}

impl<'a> ValidationEntry<'a> for module::Module {
    type ValidationResult = <module::Module as Validate<'a>>::ValidationResult;
    fn validate(&'a self) -> Result<Self::ValidationResult> {
        let mut ctx = Context::default();
        <module::Module as Validate<'a>>::validate(self, &mut ctx)
    }
}

impl<'a> Validate<'a> for module::Module {
    type ValidationResult = result::Module<'a>;
    fn validate(&'a self, _ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        let mut ctx = Context::from(&self)?;
        let mut ctx_globals = Context::default();
        ctx_globals.use_globals(&self.globals);

        for ty in self.types.iter() {
            ty.validate(&mut ctx)?;
        }
        for func in self.funcs.iter() {
            func.validate(&mut ctx)?;
        }
        for table in self.tables.iter() {
            table.validate(&mut ctx)?;
        }
        for mem in self.mems.iter() {
            mem.validate(&mut ctx)?;
        }
        for global in self.globals.iter() {
            global.validate(&mut ctx_globals)?;
        }
        for elem in self.elem.iter() {
            elem.validate(&mut ctx)?;
        }
        for data in self.data.iter() {
            data.validate(&mut ctx)?;
        }
        if let Some(ref start) = self.start {
            start.validate(&mut ctx)?;
        }
        let mut imports = Vec::with_capacity(self.imports.len());
        for import in self.imports.iter() {
            let ty = import.validate(&mut ctx)?;
            imports.push(ty);
        }
        let mut exports = Vec::with_capacity(self.exports.len());
        for export in self.exports.iter() {
            let ty = export.validate(&mut ctx)?;
            exports.push(ty);
        }

        if !(self.tables.len() <= 1) {
            return Err(Error::UnexpectedEndOfFile);
        }
        if !(self.mems.len() <= 1) {
            return Err(Error::UnexpectedEndOfFile);
        }

        let mut export_names = self.exports.iter()
            .map(|e| &e.name)
            .collect::<Vec<_>>();
        export_names.sort();
        export_names.dedup();

        if export_names.len() != self.exports.len() {
            return Err(Error::UnexpectedEndOfFile);
        }

        Ok(Self::ValidationResult {
            imports,
            exports,
        })
    }
}

impl<'a> Validate<'a> for module::Func {
    type ValidationResult = &'a ty::Func;
    fn validate(&'a self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        let ty = self.ty.validate(ctx)?;
        let ret = &ty.results[..];

        ctx.ret = Some(ret);
        ctx.locals = ty.params.iter().chain(self.locals.iter()).collect();
        ctx.stacks.push_frame(ret, ret);
        self.body.validate(ctx)?;
        ctx.stacks.pop_frame()?;
        if !ctx.stacks.frames.is_empty() || !ctx.stacks.operands.is_empty() {
            return Err(Error::UnexpectedEndOfFile);
        }
        ctx.locals.clear();
        ctx.ret = None;

        Ok(ty)
    }
}

impl<'a> Validate<'a> for module::Table {
    type ValidationResult = &'a ty::Table;
    fn validate(&'a self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        self.ty.validate(ctx)?;
        Ok(&self.ty)
    }
}

impl<'a> Validate<'a> for module::Mem {
    type ValidationResult = &'a ty::Mem;
    fn validate(&'a self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        self.ty.validate(ctx)?;
        Ok(&self.ty)
    }
}

impl<'a> Validate<'a> for module::Global {
    type ValidationResult = &'a ty::Global;
    fn validate(&'a self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        self.ty.validate(ctx)?;
        let ret = std::slice::from_ref(&self.ty.val);

        ctx.stacks.push_frame(ret, ret);
        self.init.validate(ctx)?;
        ctx.stacks.pop_frame()?;
        if !ctx.stacks.frames.is_empty() || !ctx.stacks.operands.is_empty() {
            return Err(Error::UnexpectedEndOfFile);
        }

        Ok(&self.ty)
    }
}

impl<'a> Validate<'a> for module::Elem {
    type ValidationResult = ();
    fn validate(&'a self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        let ty = self.table.validate(ctx)?;
        if ty.elem != ty::Elem::FuncRef {
            return Err(Error::UnexpectedEndOfFile);
        }

        let ret = &[ty::Val::I32];
        ctx.stacks.push_frame(ret, ret);
        self.offset.validate(ctx)?;
        ctx.stacks.pop_frame()?;
        if !ctx.stacks.frames.is_empty() || !ctx.stacks.operands.is_empty() {
            return Err(Error::UnexpectedEndOfFile);
        }

        for fcn in self.init.iter() {
            fcn.validate(ctx)?;
        }
        Ok(())
    }
}

impl<'a> Validate<'a> for module::Data {
    type ValidationResult = ();
    fn validate(&'a self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        self.mem.validate(ctx)?;

        let ret = &[ty::Val::I32];
        ctx.stacks.push_frame(ret, ret);
        self.offset.validate(ctx)?;
        ctx.stacks.pop_frame()?;
        if !ctx.stacks.frames.is_empty() || !ctx.stacks.operands.is_empty() {
            return Err(Error::UnexpectedEndOfFile);
        }

        Ok(())
    }
}

impl<'a> Validate<'a> for module::Start {
    type ValidationResult = ();
    fn validate(&'a self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        let ty = self.func.validate(ctx)?;
        if ty.params != [] || ty.results != [] {
            return Err(Error::UnexpectedEndOfFile);
        }
        Ok(())
    }
}

impl<'a> Validate<'a> for module::desc::Import {
    type ValidationResult = result::External<'a>;
    fn validate(&'a self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        match self {
            Self::Func(idx) => {
                let ty = idx.validate(ctx)?;
                Ok(Self::ValidationResult::Func( ty ))
            },
            Self::Table(ty) => {
                ty.validate(ctx)?;
                Ok(Self::ValidationResult::Table( ty ))
            },
            Self::Mem(ty) => {
                ty.validate(ctx)?;
                Ok(Self::ValidationResult::Mem( ty ))
            },
            Self::Global(ty) => {
                ty.validate(ctx)?;
                Ok(Self::ValidationResult::Global( ty ))
            },
        }
    }
}

impl<'a> Validate<'a> for module::desc::Export {
    type ValidationResult = result::External<'a>;
    fn validate(&'a self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        match self {
            Self::Func(idx) => {
                let ty = idx.validate(ctx)?;
                Ok(Self::ValidationResult::Func( ty ))
            },
            Self::Table(idx) => {
                let ty = idx.validate(ctx)?;
                Ok(Self::ValidationResult::Table( ty ))
            },
            Self::Mem(idx) => {
                let ty = idx.validate(ctx)?;
                Ok(Self::ValidationResult::Mem( ty ))
            },
            Self::Global(idx) => {
                let ty = idx.validate(ctx)?;
                Ok(Self::ValidationResult::Global( ty ))
            },
        }
    }
}

impl<'a> Validate<'a> for module::Import {
    type ValidationResult = result::External<'a>;
    fn validate(&'a self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        self.desc.validate(ctx)
    }
}

impl<'a> Validate<'a> for module::Export {
    type ValidationResult = result::External<'a>;
    fn validate(&'a self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        self.desc.validate(ctx)
    }
}