use fjall::Slice;

#[derive(clap::ValueEnum, Copy, Clone, Debug)]
#[clap(rename_all = "kebab")]
pub enum PrefixKind {
    LenU64Le,
    LenU64Be,
}

impl PrefixKind {
    pub fn write(self, slice: &Slice) -> Vec<u8> {
        use PrefixKind::*;
        let len = slice.len();
        match self {
            LenU64Le => (len as u64).to_le_bytes().to_vec(),
            LenU64Be => (len as u64).to_be_bytes().to_vec(),
        }
    }
}
