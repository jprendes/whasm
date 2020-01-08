use crate::validation::{Validate, Context, Result, Error, stacks};
use crate::structure::{idx, instr, ty};
use crate::structure::instr::Instr;

impl<'a> Validate<'a> for instr::Expr {
    type ValidationResult = ();
    fn validate(&'a self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        let instr::Expr(instrs) = self;
        for instr in instrs.iter() {
            instr.validate(ctx)?;
        }
        Ok(())
    }
}

impl<'a> Validate<'a> for instr::ConstExpr {
    type ValidationResult = ();
    fn validate(&'a self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        let instr::ConstExpr(instrs) = self;
        for instr in instrs.iter() {
            match instr {
                Instr::ConstI32(_) => (),
                Instr::ConstI64(_) => (),
                Instr::ConstF32(_) => (),
                Instr::ConstF64(_) => (),
                Instr::GlobalGet(idx) => {
                    let ty = idx.validate(ctx)?;
                    if ty.mt != ty::Mut::Const {
                        return Err(Error::UnexpectedEndOfFile);
                    }
                }
                _ => {
                    return Err(Error::UnexpectedEndOfFile);
                }
            };
            instr.validate(ctx)?
        }
        Ok(())
    }
}

impl<'a> Validate<'a> for instr::Instr {
    type ValidationResult = ();
    fn validate(&'a self, ctx: &mut Context<'a>) -> Result<Self::ValidationResult> {
        match self {
            // Control flow
            /* 0x00 */ Instr::Unreachable => {
                ctx.stacks.unreachable()?;
            },
            /* 0x01 */ Instr::Nop => {},
            /* 0x02 */ Instr::Block(ret, expr) => {
                let ret = ret.validate(ctx)?;
                ctx.stacks.push_frame(ret, ret);
                expr.validate(ctx)?;
                Instr::End.validate(ctx)?;
            },
            /* 0x03 */ Instr::Loop(ret, expr) => {
                let ret = ret.validate(ctx)?;
                ctx.stacks.push_frame(&[], ret);
                expr.validate(ctx)?;
                Instr::End.validate(ctx)?;
            },
            /* 0x04 */ Instr::If(ret, expr1, expr2) => {
                let ret = ret.validate(ctx)?;
                ctx.stacks.pop_operands(&[ty::Val::I32])?;
                ctx.stacks.push_frame(ret, ret);
                expr1.validate(ctx)?;
                Instr::Else.validate(ctx)?;
                expr2.validate(ctx)?;
                Instr::End.validate(ctx)?;
            },
            /* 0x05 */ Instr::Else => {
                let ret = ctx.stacks.pop_frame()?;
                ctx.stacks.push_frame(ret, ret);
            }
            // ... reserved ...
            /* 0x0B */ Instr::End => {
                let ret = ctx.stacks.pop_frame()?;
                ctx.stacks.push_operands(&ret[..]);
            }
            /* 0x0C */ Instr::Br(label) => {
                let frame = label.validate(ctx)?;
                ctx.stacks.pop_operands(frame.label)?;
                ctx.stacks.unreachable()?;
            }
            /* 0x0D */ Instr::BrIf(label) => {
                let frame = label.validate(ctx)?;
                ctx.stacks.pop_operands(&[ty::Val::I32])?;
                ctx.stacks.pop_operands(frame.label)?;
                ctx.stacks.push_operands(frame.label);
            }
            /* 0x0E */ Instr::BrTable(idxs, label) => {
                let frame = label.validate(ctx)?;
                for lbl in idxs.iter() {
                    let frm = lbl.validate(ctx)?;
                    if frame.label != frm.label {
                        return Err(Error::UnexpectedEndOfFile);
                    }
                }
                ctx.stacks.pop_operands(&[ty::Val::I32])?;
                ctx.stacks.pop_operands(frame.label)?;
                ctx.stacks.unreachable()?;
            }
            /* 0x0F */ Instr::Return => {
                let ret = ctx.ret.ok_or(Error::UnexpectedEndOfFile)?;
                ctx.stacks.pop_operands(&ret[..])?;
                ctx.stacks.unreachable()?;
            }
            /* 0x10 */ Instr::Call(func) => {
                let func = func.validate(ctx)?;
                ctx.stacks.pop_operands(&func.params[..])?;
                ctx.stacks.push_operands(&func.results[..]);
            }
            /* 0x11 */ Instr::CallIndirect(func,table) => {
                let table = table.validate(ctx)?;
                let func = func.validate(ctx)?;
                if table.elem != ty::Elem::FuncRef {
                    return Err(Error::UnexpectedEndOfFile);
                }
                ctx.stacks.pop_operands(&[ty::Val::I32])?;
                ctx.stacks.pop_operands(&func.params[..])?;
                ctx.stacks.push_operands(&func.results[..]);
            }
            // ... reserved ...
            // Parametric
            /* 0x1A */ Instr::Drop => {
                ctx.stacks.pop_operand(stacks::Operand::Unknown)?;
            }
            /* 0x1B */ Instr::Select => {
                ctx.stacks.pop_operands(&[ty::Val::I32])?;
                let t1 = ctx.stacks.pop_operand(stacks::Operand::Unknown)?;
                let t2 = ctx.stacks.pop_operand(t1)?;
                ctx.stacks.push_operand(t2);
            }
            // ... reserved ...
            // Variable
            /* 0x20 */ Instr::LocalGet(idx) => {
                let ty = idx.validate(ctx)?;
                ctx.stacks.push_operands(&[*ty]);
            }
            /* 0x21 */ Instr::LocalSet(idx) => {
                let ty = idx.validate(ctx)?;
                ctx.stacks.pop_operands(&[*ty])?;
            }
            /* 0x22 */ Instr::LocalTee(idx) => {
                let ty = idx.validate(ctx)?;
                ctx.stacks.pop_operands(&[*ty])?;
                ctx.stacks.push_operands(&[*ty]);
            }
            /* 0x23 */ Instr::GlobalGet(idx) => {
                let ty = idx.validate(ctx)?;
                ctx.stacks.push_operands(&[ty.val]);
            }
            /* 0x24 */ Instr::GlobalSet(idx) => {
                let ty = idx.validate(ctx)?;
                if ty.mt != ty::Mut::Var {
                    return Err(Error::UnexpectedEndOfFile);
                }
                ctx.stacks.pop_operands(&[ty.val])?;
            }
            // ... reserved ...
            // Memory load
            /* 0x28 */ Instr::I32Load(_offset, align) => {
                validate_load(ctx, *align, 32, ty::Val::I32)?;
            }
            /* 0x29 */ Instr::I64Load(_offset, align) => {
                validate_load(ctx, *align, 64, ty::Val::I64)?;
            }
            /* 0x2A */ Instr::F32Load(_offset, align) => {
                validate_load(ctx, *align, 32, ty::Val::F32)?;
            }
            /* 0x2B */ Instr::F64Load(_offset, align) => {
                validate_load(ctx, *align, 64, ty::Val::F64)?;
            }
            /* 0x2C */ Instr::I32Load8S(_offset, align) => {
                validate_load(ctx, *align, 8, ty::Val::I32)?;
            }
            /* 0x2D */ Instr::I32Load8U(_offset, align) => {
                validate_load(ctx, *align, 8, ty::Val::I32)?;
            }
            /* 0x2E */ Instr::I32Load16S(_offset, align) => {
                validate_load(ctx, *align, 16, ty::Val::I32)?;
            }
            /* 0x2F */ Instr::I32Load16U(_offset, align) => {
                validate_load(ctx, *align, 16, ty::Val::I32)?;
            }
            /* 0x30 */ Instr::I64Load8S(_offset, align) => {
                validate_load(ctx, *align, 8, ty::Val::I64)?;
            }
            /* 0x31 */ Instr::I64Load8U(_offset, align) => {
                validate_load(ctx, *align, 8, ty::Val::I64)?;
            }
            /* 0x32 */ Instr::I64Load16S(_offset, align) => {
                validate_load(ctx, *align, 16, ty::Val::I64)?;
            }
            /* 0x33 */ Instr::I64Load16U(_offset, align) => {
                validate_load(ctx, *align, 16, ty::Val::I64)?;
            }
            /* 0x34 */ Instr::I64Load32S(_offset, align) => {
                validate_load(ctx, *align, 32, ty::Val::I64)?;
            }
            /* 0x35 */ Instr::I64Load32U(_offset, align) => {
                validate_load(ctx, *align, 32, ty::Val::I64)?;
            }
            // Memory store
            /* 0x36 */ Instr::I32Store(_offset, align) => {
                validate_store(ctx, *align, 32, ty::Val::I32)?;
            }
            /* 0x37 */ Instr::I64Store(_offset, align) => {
                validate_store(ctx, *align, 64, ty::Val::I64)?;
            }
            /* 0x38 */ Instr::F32Store(_offset, align) => {
                validate_store(ctx, *align, 32, ty::Val::F32)?;
            }
            /* 0x39 */ Instr::F64Store(_offset, align) => {
                validate_store(ctx, *align, 64, ty::Val::F64)?;
            }
            /* 0x3A */ Instr::I32Store8(_offset, align) => {
                validate_store(ctx, *align, 8, ty::Val::I32)?;
            }
            /* 0x3B */ Instr::I32Store16(_offset, align) => {
                validate_store(ctx, *align, 16, ty::Val::I32)?;
            }
            /* 0x3C */ Instr::I64Store8(_offset, align) => {
                validate_store(ctx, *align, 8, ty::Val::I64)?;
            }
            /* 0x3D */ Instr::I64Store16(_offset, align) => {
                validate_store(ctx, *align, 16, ty::Val::I64)?;
            }
            /* 0x3E */ Instr::I64Store32(_offset, align) => {
                validate_store(ctx, *align, 32, ty::Val::I64)?;
            }
            // Memory management
            /* 0x3F */ Instr::MemSize(idx) => {
                idx.validate(ctx)?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x40 */ Instr::MemGrow(idx) => {
                idx.validate(ctx)?;
                ctx.stacks.pop_operands(&[ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            // Numeric
            /* 0x41 */ Instr::ConstI32(_) => {
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x42 */ Instr::ConstI64(_) => {
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0x43 */ Instr::ConstF32(_) => {
                ctx.stacks.push_operands(&[ty::Val::F32]);
            }
            /* 0x44 */ Instr::ConstF64(_) => {
                ctx.stacks.push_operands(&[ty::Val::F64]);
            }
            // Numeric without immediate -- i32
            /* 0x45 */ Instr::I32Eqz => {
                ctx.stacks.pop_operands(&[ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x46 */ Instr::I32Eq => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x47 */ Instr::I32Ne => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x48 */ Instr::I32LtS => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x49 */ Instr::I32LtU => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x4A */ Instr::I32GtS => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x4B */ Instr::I32GtU => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x4C */ Instr::I32LeS => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x4D */ Instr::I32LeU => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x4E */ Instr::I32GeS => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x4F */ Instr::I32GeU => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            // Numeric without immediate -- i64
            /* 0x50 */ Instr::I64Eqz => {
                ctx.stacks.pop_operands(&[ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x51 */ Instr::I64Eq => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x52 */ Instr::I64Ne => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x53 */ Instr::I64LtS => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x54 */ Instr::I64LtU => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x55 */ Instr::I64GtS => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x56 */ Instr::I64GtU => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x57 */ Instr::I64LeS => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x58 */ Instr::I64LeU => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x59 */ Instr::I64GeS => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x5A */ Instr::I64GeU => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            // Numeric without immediate -- f32
            /* 0x5B */ Instr::F32Eq => {
                ctx.stacks.pop_operands(&[ty::Val::F32, ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x5C */ Instr::F32Ne => {
                ctx.stacks.pop_operands(&[ty::Val::F32, ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x5D */ Instr::F32Lt => {
                ctx.stacks.pop_operands(&[ty::Val::F32, ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x5E */ Instr::F32Gt => {
                ctx.stacks.pop_operands(&[ty::Val::F32, ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x5F */ Instr::F32Le => {
                ctx.stacks.pop_operands(&[ty::Val::F32, ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x60 */ Instr::F32Ge => {
                ctx.stacks.pop_operands(&[ty::Val::F32, ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            // Numeric without immediate -- f64
            /* 0x61 */ Instr::F64Eq => {
                ctx.stacks.pop_operands(&[ty::Val::F64, ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x62 */ Instr::F64Ne => {
                ctx.stacks.pop_operands(&[ty::Val::F64, ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x63 */ Instr::F64Lt => {
                ctx.stacks.pop_operands(&[ty::Val::F64, ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x64 */ Instr::F64Gt => {
                ctx.stacks.pop_operands(&[ty::Val::F64, ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x65 */ Instr::F64Le => {
                ctx.stacks.pop_operands(&[ty::Val::F64, ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x66 */ Instr::F64Ge => {
                ctx.stacks.pop_operands(&[ty::Val::F64, ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            // Numeric without immediate -- i32 cont.
            /* 0x67 */ Instr::I32Clz => {
                ctx.stacks.pop_operands(&[ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x68 */ Instr::I32Ctz => {
                ctx.stacks.pop_operands(&[ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x69 */ Instr::I32Popcnt => {
                ctx.stacks.pop_operands(&[ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x6A */ Instr::I32Add => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x6B */ Instr::I32Sub => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x6C */ Instr::I32Mul => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x6D */ Instr::I32DivS => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x6E */ Instr::I32DivU => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x6F */ Instr::I32RemS => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x70 */ Instr::I32RemU => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x71 */ Instr::I32And => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x72 */ Instr::I32Or => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x73 */ Instr::I32Xor => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x74 */ Instr::I32Shl => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x75 */ Instr::I32ShrS => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x76 */ Instr::I32ShrU => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x77 */ Instr::I32Rotl => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0x78 */ Instr::I32Rotr => {
                ctx.stacks.pop_operands(&[ty::Val::I32, ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            // Numeric without immediate -- i64 cont.
            /* 0x79 */ Instr::I64Clz => {
                ctx.stacks.pop_operands(&[ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0x7A */ Instr::I64Ctz => {
                ctx.stacks.pop_operands(&[ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0x7B */ Instr::I64Popcnt => {
                ctx.stacks.pop_operands(&[ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0x7C */ Instr::I64Add => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0x7D */ Instr::I64Sub => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0x7E */ Instr::I64Mul => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0x7F */ Instr::I64DivS => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0x80 */ Instr::I64DivU => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0x81 */ Instr::I64RemS => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0x82 */ Instr::I64RemU => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0x83 */ Instr::I64And => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0x84 */ Instr::I64Or => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0x85 */ Instr::I64Xor => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0x86 */ Instr::I64Shl => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0x87 */ Instr::I64ShrS => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0x88 */ Instr::I64ShrU => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0x89 */ Instr::I64Rotl => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0x8A */ Instr::I64Rotr => {
                ctx.stacks.pop_operands(&[ty::Val::I64, ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            // Numeric without immediate -- f32 cont.
            /* 0x8B */ Instr::F32Abs => {
                ctx.stacks.pop_operands(&[ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::F32]);
            }
            /* 0x8C */ Instr::F32Neg => {
                ctx.stacks.pop_operands(&[ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::F32]);
            }
            /* 0x8D */ Instr::F32Ceil => {
                ctx.stacks.pop_operands(&[ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::F32]);
            }
            /* 0x8E */ Instr::F32Floor => {
                ctx.stacks.pop_operands(&[ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::F32]);
            }
            /* 0x8F */ Instr::F32Trunc => {
                ctx.stacks.pop_operands(&[ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::F32]);
            }
            /* 0x90 */ Instr::F32Nearest => {
                ctx.stacks.pop_operands(&[ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::F32]);
            }
            /* 0x91 */ Instr::F32Sqrt => {
                ctx.stacks.pop_operands(&[ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::F32]);
            }
            /* 0x92 */ Instr::F32Add => {
                ctx.stacks.pop_operands(&[ty::Val::F32, ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::F32]);
            }
            /* 0x93 */ Instr::F32Sub => {
                ctx.stacks.pop_operands(&[ty::Val::F32, ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::F32]);
            }
            /* 0x94 */ Instr::F32Mul => {
                ctx.stacks.pop_operands(&[ty::Val::F32, ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::F32]);
            }
            /* 0x95 */ Instr::F32Div => {
                ctx.stacks.pop_operands(&[ty::Val::F32, ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::F32]);
            }
            /* 0x96 */ Instr::F32Min => {
                ctx.stacks.pop_operands(&[ty::Val::F32, ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::F32]);
            }
            /* 0x97 */ Instr::F32Max => {
                ctx.stacks.pop_operands(&[ty::Val::F32, ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::F32]);
            }
            /* 0x98 */ Instr::F32Copysign => {
                ctx.stacks.pop_operands(&[ty::Val::F32, ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::F32]);
            }
            // Numeric without immediate -- f64 cont.
            /* 0x99 */ Instr::F64Abs => {
                ctx.stacks.pop_operands(&[ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::F64]);
            }
            /* 0x9A */ Instr::F64Neg => {
                ctx.stacks.pop_operands(&[ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::F64]);
            }
            /* 0x9B */ Instr::F64Ceil => {
                ctx.stacks.pop_operands(&[ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::F64]);
            }
            /* 0x9C */ Instr::F64Floor => {
                ctx.stacks.pop_operands(&[ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::F64]);
            }
            /* 0x9D */ Instr::F64Trunc => {
                ctx.stacks.pop_operands(&[ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::F64]);
            }
            /* 0x9E */ Instr::F64Nearest => {
                ctx.stacks.pop_operands(&[ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::F64]);
            }
            /* 0x9F */ Instr::F64Sqrt => {
                ctx.stacks.pop_operands(&[ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::F64]);
            }
            /* 0xA0 */ Instr::F64Add => {
                ctx.stacks.pop_operands(&[ty::Val::F64, ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::F64]);
            }
            /* 0xA1 */ Instr::F64Sub => {
                ctx.stacks.pop_operands(&[ty::Val::F64, ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::F64]);
            }
            /* 0xA2 */ Instr::F64Mul => {
                ctx.stacks.pop_operands(&[ty::Val::F64, ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::F64]);
            }
            /* 0xA3 */ Instr::F64Div => {
                ctx.stacks.pop_operands(&[ty::Val::F64, ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::F64]);
            }
            /* 0xA4 */ Instr::F64Min => {
                ctx.stacks.pop_operands(&[ty::Val::F64, ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::F64]);
            }
            /* 0xA5 */ Instr::F64Max => {
                ctx.stacks.pop_operands(&[ty::Val::F64, ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::F64]);
            }
            /* 0xA6 */ Instr::F64Copysign => {
                ctx.stacks.pop_operands(&[ty::Val::F64, ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::F64]);
            }
            // Numeric without immediate -- conversion
            /* 0xA7 */ Instr::I32WrapI64 => {
                ctx.stacks.pop_operands(&[ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0xA8 */ Instr::I32TruncF32S => {
                ctx.stacks.pop_operands(&[ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0xA9 */ Instr::I32TruncF32U => {
                ctx.stacks.pop_operands(&[ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0xAA */ Instr::I32TruncF64S => {
                ctx.stacks.pop_operands(&[ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0xAB */ Instr::I32TruncF64U => {
                ctx.stacks.pop_operands(&[ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0xAC */ Instr::I64ExtendI32S => {
                ctx.stacks.pop_operands(&[ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0xAD */ Instr::I64ExtendI32U => {
                ctx.stacks.pop_operands(&[ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0xAE */ Instr::I64TruncF32S => {
                ctx.stacks.pop_operands(&[ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0xAF */ Instr::I64TruncF32U => {
                ctx.stacks.pop_operands(&[ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0xB0 */ Instr::I64TruncF64S => {
                ctx.stacks.pop_operands(&[ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0xB1 */ Instr::I64TruncF64U => {
                ctx.stacks.pop_operands(&[ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0xB2 */ Instr::F32ConvertI32S => {
                ctx.stacks.pop_operands(&[ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::F32]);
            }
            /* 0xB3 */ Instr::F32ConvertI32U => {
                ctx.stacks.pop_operands(&[ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::F32]);
            }
            /* 0xB4 */ Instr::F32ConvertI64S => {
                ctx.stacks.pop_operands(&[ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::F32]);
            }
            /* 0xB5 */ Instr::F32ConvertI64U => {
                ctx.stacks.pop_operands(&[ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::F32]);
            }
            /* 0xB6 */ Instr::F32DemoteF64 => {
                ctx.stacks.pop_operands(&[ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::F32]);
            }
            /* 0xB7 */ Instr::F64ConvertI32S => {
                ctx.stacks.pop_operands(&[ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::F64]);
            }
            /* 0xB8 */ Instr::F64ConvertI32U => {
                ctx.stacks.pop_operands(&[ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::F64]);
            }
            /* 0xB9 */ Instr::F64ConvertI64S => {
                ctx.stacks.pop_operands(&[ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::F64]);
            }
            /* 0xBA */ Instr::F64ConvertI64U => {
                ctx.stacks.pop_operands(&[ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::F64]);
            }
            /* 0xBB */ Instr::F64PromoteF32 => {
                ctx.stacks.pop_operands(&[ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::F64]);
            }
            /* 0xBC */ Instr::I32ReinterpretF32 => {
                ctx.stacks.pop_operands(&[ty::Val::F32])?;
                ctx.stacks.push_operands(&[ty::Val::I32]);
            }
            /* 0xBD */ Instr::I64ReinterpretF64 => {
                ctx.stacks.pop_operands(&[ty::Val::F64])?;
                ctx.stacks.push_operands(&[ty::Val::I64]);
            }
            /* 0xBE */ Instr::F32ReinterpretI32 => {
                ctx.stacks.pop_operands(&[ty::Val::I32])?;
                ctx.stacks.push_operands(&[ty::Val::F32]);
            }
            /* 0xBF */ Instr::F64ReinterpretI64 => {
                ctx.stacks.pop_operands(&[ty::Val::I64])?;
                ctx.stacks.push_operands(&[ty::Val::F64]);
            }
            // ... reserved ...
        }
        Ok(())
    }
}

fn validate_load<'a>(ctx: &mut Context<'a>, _align: u32, _size: u32, val: ty::Val) -> Result<()> {
    idx::Mem(0).validate(ctx)?;
    /*
    if !( 1<<align <= size/8 ) {
        return Err(Error::UnexpectedEndOfFile);
    }
    */
    ctx.stacks.pop_operands(&[ty::Val::I32])?;
    ctx.stacks.push_operands(&[val]);
    Ok(())
}

fn validate_store<'a>(ctx: &mut Context<'a>, _align: u32, _size: u32, val: ty::Val) -> Result<()> {
    idx::Mem(0).validate(ctx)?;
    /*
    if !( 1<<align <= size/8 ) {
        return Err(Error::UnexpectedEndOfFile);
    }
    */
    ctx.stacks.pop_operands(&[ty::Val::I32, val])?;
    Ok(())
}