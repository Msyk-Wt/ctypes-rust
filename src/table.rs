use crate::types::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::From)]
pub struct Key(pub(crate) KeyBase);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::From)]
pub(crate) enum KeyBase {

    Void,

    #[from(BasicType,
        IntegerType, RealType, SignedType, UnsignedType, FloatType,
        Int8Type, Int16Type, Int32Type, Int64Type,
        Uint8Type, Uint16Type, Uint32Type, Uint64Type,
        Float32Type, Float64Type)]
    Basic(BasicType),
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Table {
    
}
impl Table {

}
