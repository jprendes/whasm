use crate::binary::{WasmBinaryParse, WasmBinary, Result, Byte, Error};
use num_traits::*;

impl WasmBinaryParse for i8 {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        parse_signed_leb_128(bin)
    }
}

impl WasmBinaryParse for i16 {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        parse_signed_leb_128(bin)
    }
}

impl WasmBinaryParse for i32 {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        parse_signed_leb_128(bin)
    }
}

impl WasmBinaryParse for i64 {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        parse_signed_leb_128(bin)
    }
}

fn parse_signed_leb_128<T, Binary: WasmBinary>(bin: &mut Binary) -> Result<T>
where T: PrimInt + Signed + FromPrimitive {
    let size = 8 * std::mem::size_of::<T>();
    let mask = T::from_u8(0x7F).unwrap();
    let mut result = T::zero();
    let mut shift = 0;
    let mut positive = true;
    loop {
        let Byte(byte) = bin.parse()?;
        let val = T::from_u8(byte & 0x7F).unwrap();
        if shift >= size {
            match byte & 0x7F {
                0x00 if positive => (),
                0x7F if !positive => (),
                _ => Err(Error::OutOfRangeSignedInteger)?,
            }
            if byte & 0x80 == 0 {
                break;
            } else {
                continue;
            }
        }
        if ( ((val << shift) >> shift) & mask ) != val {
            Err(Error::OutOfRangeSignedInteger)?;
        }
        if shift < size {
            result = result | ( val << shift);
            shift += 7;
        }
        positive = (byte & 0x40) == 0;
        if byte & 0x80 == 0 {
            break;
        }
    }
    if ( shift < size ) && !positive {
        result = result | ( (!T::zero()) << shift );
    }
    Ok(result)
}