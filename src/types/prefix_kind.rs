use errgonomic::handle_bool;
use fjall::Slice;
use thiserror::Error;

#[derive(clap::ValueEnum, Copy, Clone, Debug)]
#[clap(rename_all = "kebab")]
pub enum PrefixKind {
    LenU32Le,
    LenU64Le,
    LenU32Be,
    LenU64Be,
}

impl PrefixKind {
    pub fn write(self, slice: &Slice) -> Result<Vec<u8>, PrefixKindWriteError> {
        use PrefixKind::*;
        use PrefixKindWriteError::*;
        let len = slice.len();
        match self {
            LenU32Le => {
                handle_bool!(len > u32::MAX as usize, LenU32Invalid, len);
                Ok((len as u32).to_le_bytes().to_vec())
            }
            LenU64Le => Ok((len as u64).to_le_bytes().to_vec()),
            LenU32Be => {
                handle_bool!(len > u32::MAX as usize, LenU32Invalid, len);
                Ok((len as u32).to_be_bytes().to_vec())
            }
            LenU64Be => Ok((len as u64).to_be_bytes().to_vec()),
        }
    }
}

#[derive(Error, Debug, Copy, Clone)]
pub enum PrefixKindWriteError {
    #[error("slice length '{len}' does not fit into u32")]
    LenU32Invalid { len: usize },
}
