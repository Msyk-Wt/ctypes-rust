use crate::{types::*, environment::*, types::*};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::From)]
pub struct Key(pub(crate) KeyBase);
impl Key {

    pub(crate) fn make_type(&self, env: Environment) -> Type {
        Type(match &self.0 {
            KeyBase::Void => TypeBase::Void,
            KeyBase::Basic(b) => TypeBase::Basic(*b),
            KeyBase::Enumeration(key) => TypeBase::Enumeration(EnumerationType::new(*key, env)),
            KeyBase::Pointer(key) => TypeBase::Pointer(PointerType::new(*key, env)),
        })
    }

    pub(crate) fn get_entry_ref<'a>(&self, table: &'a Table) -> TypeEntryRef<'a> {
        match &self.0 {
            KeyBase::Void => TypeEntryRef::Void,
            KeyBase::Basic(b) => TypeEntryRef::Basic(*b),
            KeyBase::Enumeration(i) => TypeEntryRef::Enumeration(&table.enumeration[*i]),
            KeyBase::Pointer(i) => TypeEntryRef::Pointer(&table.pointer[*i]),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::From)]
pub(crate) enum KeyBase {

    Void,

    #[from(BasicType,
        IntegerType, RealType, SignedType, UnsignedType, FloatType,
        Int8Type, Int16Type, Int32Type, Int64Type,
        Uint8Type, Uint16Type, Uint32Type, Uint64Type,
        Float32Type, Float64Type)]
    Basic(BasicType),

    Enumeration(usize),

    Pointer(usize),
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Table {
    pub enumeration: Vec<EnumerationTypeEntry>,
    pub pointer: Vec<PointerTypeEntry>,
}
