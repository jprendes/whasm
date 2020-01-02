use crate::binary::{WasmBinaryParse, WasmBinary, Result, Byte, Error};
use num_traits::*;

impl WasmBinaryParse for u8 {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        parse_unsigned_leb_128(bin)
    }
}

impl WasmBinaryParse for u16 {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        parse_unsigned_leb_128(bin)
    }
}

impl WasmBinaryParse for u32 {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        parse_unsigned_leb_128(bin)
    }
}

impl WasmBinaryParse for u64 {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        parse_unsigned_leb_128(bin)
    }
}

fn parse_unsigned_leb_128<T, Binary: WasmBinary>(bin: &mut Binary) -> Result<T>
where T: PrimInt + Unsigned + FromPrimitive {
    let size = 8 * std::mem::size_of::<T>();
    let mask = T::from_u8(0x7F).unwrap();
    let mut result = T::zero();
    let mut shift = 0;
    loop {
        let Byte(byte) = bin.parse()?;
        let val = T::from_u8(byte & 0x7F).unwrap();
        if shift >= size {
            if byte & 0x7F != 0 {
                Err(Error::OutOfRangeUnsignedInteger)?;
            }
            if byte & 0x80 == 0 {
                break;
            } else {
                continue;
            }
        }
        if ( ((val << shift) >> shift) & mask ) != val {
            Err(Error::OutOfRangeUnsignedInteger)?;
        }
        if shift < size {
            result = result | ( val << shift);
            shift += 7;
        }
        if byte & 0x80 == 0 {
            break;
        }
    }
    Ok(result)
}