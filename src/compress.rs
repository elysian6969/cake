pub use consts::BITS;
pub use consts::{PTR_BITS, TAG_BITS};
pub use consts::{PTR_MASK, TAG_MASK};

pub use consts::LEN_CAP_MAX;
pub use consts::{CAP_BITS, LEN_BITS};
pub use consts::{CAP_MASK, LEN_MASK};

pub use self::str::Str;
pub use licity::Licity;
pub use slice::Slice;
pub use tagged::WithTag;

mod consts;
mod licity;
mod slice;
mod str;
mod tagged;
