use crate::binary::{WasmBinaryParse, WasmBinary, Result, core::proxy::WasmBinaryParseProxy};

impl<T: WasmBinaryParse> WasmBinaryParse for Vec<T> {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let n: u32 = bin.parse()?;
        let mut result = Vec::with_capacity(n as usize);
        for _ in 0..n {
            result.push(bin.parse()?);
        }
        Ok(result)
    }
}

pub struct CompactVec<T> ( Vec<T> );
impl<T: WasmBinaryParse + Copy> WasmBinaryParseProxy for CompactVec<T> {
    type Inner = Vec<T>;

    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let mut result = vec![];
        let k: u32 = bin.parse()?;
        for _ in 0..k {
            let n: u32 = bin.parse()?;
            let val = bin.parse()?;
            for _ in 0..n {
                result.push(val);
            }
        }
        Ok(Self(result))
    }

    fn unwrap(self) -> Self::Inner { self.0 }
}

pub struct UnwrappingVec<T: WasmBinaryParseProxy> ( Vec<T::Inner> );
impl<T: WasmBinaryParseProxy> WasmBinaryParseProxy for UnwrappingVec<T> {
    type Inner = Vec<T::Inner>;

    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let n: u32 = bin.parse()?;
        let mut result = Vec::with_capacity(n as usize);
        for _ in 0..n {
            result.push(<T as WasmBinaryParseProxy>::parse(bin)?.unwrap());
        }
        Ok(Self(result))
    }

    fn unwrap(self) -> Self::Inner { self.0 }
}