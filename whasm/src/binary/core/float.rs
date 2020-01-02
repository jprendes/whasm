use crate::binary::{WasmBinaryParse, WasmBinary, Result, Byte};
use num_traits::*;

impl WasmBinaryParse for f32 {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        parse_float_iee754(bin)
    }
}

impl WasmBinaryParse for f64 {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        parse_float_iee754(bin)
    }
}

fn parse_float_iee754<T, Binary: WasmBinary>(bin: &mut Binary) -> Result<T>
where T: Float + Sized {
    let mut result = T::zero();
    let bytes: *mut u8 = (&mut result as *mut T).cast();
    for n in 0..std::mem::size_of::<T>() {
        let Byte(byte) = bin.parse()?;
        unsafe { bytes.offset(n as isize).write_unaligned(byte) };
    }
    Ok(result)
}