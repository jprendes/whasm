use crate::validation::{Validate, Result, stacks};
use crate::structure::{module, ty};

#[derive(Default)]
pub struct Context<'a> {
    pub types: Vec<&'a ty::Func>,
    pub funcs: Vec<&'a ty::Func>,
    pub tables: Vec<&'a ty::Table>,
    pub mems: Vec<&'a ty::Mem>,
    pub globals: Vec<&'a ty::Global>,
    pub locals: Vec<&'a ty::Val>,
    pub labels: Vec<&'a ty::RetVal>,
    pub ret: Option<&'a [ty::Val]>,
    pub stacks: stacks::Stacks<'a>,
}

impl<'a> Context<'a> {
    pub fn use_types(&mut self, types: &'a Vec<ty::Func>) {
        for ty in types.iter() {
            self.types.push(&ty);
        }
    }

    pub fn use_imported_funcs(&mut self, imports: &'a Vec<module::Import>) -> Result<()> {
        for import in imports.iter() {
            if let module::desc::Import::Func(idx) = &import.desc {
                let ty = idx.validate(self)?;
                self.funcs.push(ty);
            }
        }
        Ok(())
    }

    pub fn use_imported_tables(&mut self, imports: &'a Vec<module::Import>) {
        for import in imports.iter() {
            if let module::desc::Import::Table(ty) = &import.desc {
                self.tables.push(&ty)
            }
        }
    }

    pub fn use_imported_mems(&mut self, imports: &'a Vec<module::Import>) {
        for import in imports.iter() {
            if let module::desc::Import::Mem(ty) = &import.desc {
                self.mems.push(&ty)
            }
        }
    }

    pub fn use_imported_globals(&mut self, imports: &'a Vec<module::Import>) {
        for import in imports.iter() {
            if let module::desc::Import::Global(ty) = &import.desc {
                self.globals.push(&ty)
            }
        }
    }

    pub fn use_funcs(&mut self, funcs: &'a Vec<module::Func>) -> Result<()> {
        for func in funcs.iter() {
            let ty = func.ty.validate(self)?;
            self.funcs.push(ty);
        }
        Ok(())
    }

    pub fn use_tables(&mut self, tables: &'a Vec<module::Table>) {
        for table in tables.iter() {
            self.tables.push(&table.ty);
        }
    }

    pub fn use_mems(&mut self, mems: &'a Vec<module::Mem>) {
        for mem in mems.iter() {
            self.mems.push(&mem.ty);
        }
    }

    pub fn use_globals(&mut self, globals: &'a Vec<module::Global>) {
        for global in globals.iter() {
            self.globals.push(&global.ty);
        }
    }

    pub fn from(mdl: &'a module::Module) -> Result<Context<'a>> {
        let mut ctx = Context::default();

        ctx.use_types(&mdl.types);
        ctx.use_imported_funcs(&mdl.imports)?;
        ctx.use_imported_tables(&mdl.imports);
        ctx.use_imported_mems(&mdl.imports);
        ctx.use_imported_globals(&mdl.imports);
        ctx.use_funcs(&mdl.funcs)?;
        ctx.use_tables(&mdl.tables);
        ctx.use_mems(&mdl.mems);
        ctx.use_globals(&mdl.globals);

        Ok(ctx)
    }
}