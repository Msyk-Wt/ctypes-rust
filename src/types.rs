mod basic;
mod enumeration;
mod pointer;

pub use basic::*;
pub use enumeration::*;
pub use pointer::*;
use crate::{environment::*, table::*};

#[derive(Debug, Clone, derive_more::Display, derive_more::From)]
pub struct Type(pub(crate) TypeBase);
impl Type {

    pub fn byte_size(&self) -> usize {
        match &self.0 {
            TypeBase::Void => 0,
            TypeBase::Basic(b) => b.byte_size(),
            TypeBase::Enumeration(e) => e.byte_size(),
            TypeBase::Pointer(p) => p.byte_size(),
        }
    }

    pub(crate) fn key(&self) -> Key {
        match &self.0 {
            TypeBase::Void => Key(KeyBase::Void),
            TypeBase::Basic(b) => Key(KeyBase::Basic(*b)),
            TypeBase::Enumeration(e) => Key(KeyBase::Enumeration(e.key)),
            TypeBase::Pointer(p) => Key(KeyBase::Pointer(p.key)),
        }
    }
}

#[derive(Debug, Clone, derive_more::Display, derive_more::From)]
pub(crate) enum TypeBase {
    Void,
    Basic(BasicType),
    Enumeration(EnumerationType),
    Pointer(PointerType),
}

#[derive(Debug, Clone, derive_more::From)]
pub(crate) enum TypeEntryRef<'a> {
    Void,
    Basic(BasicType),
    Enumeration(&'a EnumerationTypeEntry),
    Pointer(&'a PointerTypeEntry),
}
impl<'a> TypeEntryRef<'a> {

    pub(crate) fn display(&self, f: &mut core::fmt::Formatter<'_>, env: &EnvBase) -> core::fmt::Result {
        match self {
            Self::Void => write!(f, "void"),
            Self::Basic(t) => write!(f, "{}", t.name()),
            Self::Enumeration(e) => e.display(f),
            Self::Pointer(e) => e.display(f, env),
        }
    }
}
