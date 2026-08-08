
#[cfg(not(feature="arc"))]
use std::{rc::Rc, cell::{RefCell, Ref, RefMut}};
#[cfg(feature="arc")]
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{Endian, error::*, table::*, types::*};


#[cfg(not(feature="arc"))]
/// データ型管理環境
#[derive(Debug, Clone)]
pub struct Environment(Rc<RefCell<EnvBase>>);

#[cfg(feature="arc")]
/// データ型管理環境
#[derive(Debug, Clone)]
pub struct Environment(Arc<RwLock<EnvBase>>);

impl Environment {

    pub fn new(address_size: usize, default_endian: Endian) -> Result<Self, Error> {

        let env = EnvBase {
            address_type: UnsignedType::from_byte_size(address_size, default_endian)?,
            default_endian,
            type_table: Table::default(),
        };

        #[cfg(not(feature="arc"))]
        let env = Rc::new(RefCell::new(env));
        #[cfg(feature="arc")]
        let env = Arc::new(RwLock::new(env));

        Ok(Self(env))
    }

    pub fn address_size(&self) -> usize {
        self.as_ref().address_type.byte_size()
    }

    pub fn default_endian(&self) -> Endian {
        self.as_ref().default_endian
    }

    pub fn i8(&self) -> Int8Type {
        Int8Type::new(self.default_endian())
    }

    pub fn i16(&self) -> Int16Type {
        Int16Type::new(self.default_endian())
    }

    pub fn i32(&self) -> Int32Type {
        Int32Type::new(self.default_endian())
    }

    pub fn i64(&self) -> Int64Type {
        Int64Type::new(self.default_endian())
    }

    pub fn u8(&self) -> Uint8Type {
        Uint8Type::new(self.default_endian())
    }

    pub fn u16(&self) -> Uint16Type {
        Uint16Type::new(self.default_endian())
    }

    pub fn u32(&self) -> Uint32Type {
        Uint32Type::new(self.default_endian())
    }

    pub fn u64(&self) -> Uint64Type {
        Uint64Type::new(self.default_endian())
    }

    pub fn f32(&self) -> Float32Type {
        Float32Type::new(self.default_endian())
    }

    pub fn f64(&self) -> Float64Type {
        Float64Type::new(self.default_endian())
    }

    #[cfg(not(feature="arc"))]
    pub(crate) fn as_ref(&self) -> Ref<'_, EnvBase> {
        self.0.borrow()
    }
    #[cfg(feature="arc")]
    pub(crate) fn as_ref(&self) -> RwLockReadGuard<'_, EnvBase> {
        self.0.read().unwrap_or_else(|e| e.into_inner())
    }

    #[cfg(not(feature="arc"))]
    pub(crate) fn as_mut(&self) -> RefMut<'_, EnvBase> {
        self.0.borrow_mut()
    }
    #[cfg(feature="arc")]
    pub(crate) fn as_mut(&self) -> RwLockWriteGuard<'_, EnvBase> {
        self.0.write().unwrap_or_else(|e| e.into_inner())
    }
}


#[derive(Debug, Clone)]
pub(crate) struct EnvBase {
    pub address_type: UnsignedType,
    pub default_endian: Endian,
    pub type_table: Table,
}
