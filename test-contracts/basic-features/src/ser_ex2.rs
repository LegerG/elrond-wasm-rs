use elrond_wasm::elrond_codec::*;

/// Copied from elrond-wasm serialization tests. 
pub enum SerExample2 {
    Unit,
    Newtype(u32),
    Tuple(u32, u32),
    Struct { a: u32 },
}

impl NestedEncode for SerExample2 {
    fn dep_encode_to<O: NestedOutputBuffer>(&self, dest: &mut O) -> Result<(), EncodeError> {
        match self {
            SerExample2::Unit => {
                0u32.dep_encode_to(dest)?;
            },
            SerExample2::Newtype(arg1) => {
                1u32.dep_encode_to(dest)?;
                arg1.dep_encode_to(dest)?;
            },
            SerExample2::Tuple(arg1, arg2) => {
                2u32.dep_encode_to(dest)?;
                arg1.dep_encode_to(dest)?;
                arg2.dep_encode_to(dest)?;
            },
            SerExample2::Struct { a } => {
                3u32.dep_encode_to(dest)?;
                a.dep_encode_to(dest)?;
            },
        }
        Ok(())
    }
}

impl TopEncode for SerExample2 {
    fn top_encode<B: TopEncodeBuffer, O: TopEncodeOutput<B>>(&self, output: O) -> Result<(), EncodeError> {
        let mut buffer = output.into_output_buffer();
        self.dep_encode_to(&mut buffer)?;
        buffer.save_buffer();
        Ok(())
    }
}

impl NestedDecode for SerExample2 {
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        match u32::dep_decode(input)? {
            0 => Ok(SerExample2::Unit),
            1 => Ok(SerExample2::Newtype(u32::dep_decode(input)?)),
            2 => Ok(SerExample2::Tuple(u32::dep_decode(input)?, u32::dep_decode(input)?)),
            3 => Ok(SerExample2::Struct{ a: u32::dep_decode(input)? }),
            _ => Err(DecodeError::INVALID_VALUE),
        }
    }
}

impl TopDecode for SerExample2 {
    fn top_decode<I: TopDecodeInput>(mut input: I) -> Result<Self, DecodeError> {
        dep_decode_from_byte_slice(input.get_slice_u8())
    }
}
