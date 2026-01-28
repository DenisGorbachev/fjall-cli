macro_rules! impl_decode_bytes_method {
    ($fn_name:ident, $error_ty:ident) => {
        pub fn $fn_name((input, encoding): (&str, ByteEncoding)) -> Result<Vec<u8>, $error_ty> {
            use $error_ty::*;
            let bytes = handle!(encoding.decode(input), DecodeFailed);
            Ok(bytes)
        }
    };
}

mod command;
pub use command::*;

mod types;
pub use types::*;
