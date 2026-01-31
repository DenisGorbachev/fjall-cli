use crate::{PrefixKind, Suffix};

#[derive(Clone, Debug)]
pub struct OutputAffixes {
    pub item_prefix: Option<PrefixKind>,
    pub item_suffix: Option<Suffix>,
    pub key_prefix: Option<PrefixKind>,
    pub key_suffix: Option<Suffix>,
    pub value_prefix: Option<PrefixKind>,
    pub value_suffix: Option<Suffix>,
}
