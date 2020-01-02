use crate::binary::{WasmBinaryParse, WasmBinary, Result, Error, core::proxy::WasmBinaryParseProxy};

pub struct Sized<T> ( T );
impl<T: WasmBinaryParse> WasmBinaryParseProxy for Sized<T> {
    type Inner = T;

    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        let size: u32 = bin.parse()?;
        let mut count = 0;
        let mut bin = bin
            .take(size as usize)
            .inspect(|_| count = count + 1);
        let result = bin.parse()?;
        if size != count {
            Err(Error::SizeMissmatch)
        } else {
            Ok(Self(result))
        }
    }

    fn unwrap(self) -> Self::Inner { self.0 }
}

pub struct Consume { pub count: usize }
impl WasmBinaryParse for Consume {
    fn parse<Binary: WasmBinary>(bin: &mut Binary) -> Result<Self> {
        Ok(Consume { count: bin.count() })
    }
}