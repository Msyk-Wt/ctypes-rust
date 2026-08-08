
#[cfg(not(feature="arc"))]
use std::{rc::Rc, cell::{RefCell, Ref, RefMut}};
#[cfg(feature="arc")]
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::Endian;


#[cfg(not(feature="arc"))]
/// データ型管理環境
#[derive(Debug, Clone)]
pub struct Environment(Rc<RefCell<EnvBase>>);

#[cfg(feature="arc")]
/// データ型管理環境
#[derive(Debug, Clone)]
pub struct Environment(Arc<RwLock<EnvBase>>);

impl Environment {

    pub fn new(default_endian: Endian) -> Self {

        let env = EnvBase {
            default_endian,
        };

        #[cfg(not(feature="arc"))]
        let env = Rc::new(RefCell::new(env));
        #[cfg(feature="arc")]
        let env = Arc::new(RwLock::new(env));

        Self(env)
    }

    pub fn default_endian(&self) -> Endian {
        self.as_ref().default_endian
    }

    #[cfg(not(feature="arc"))]
    pub(crate) fn as_ref(&self) -> Ref<EnvBase> {
        self.0.borrow()
    }
    #[cfg(feature="arc")]
    pub(crate) fn as_ref(&self) -> RwLockReadGuard<EnvBase> {
        self.0.read().unwrap_or_else(|e| e.into_inner())
    }

    #[cfg(not(feature="arc"))]
    pub(crate) fn as_mut(&self) -> RefMut<EnvBase> {
        self.0.borrow_mut()
    }
    #[cfg(feature="arc")]
    pub(crate) fn as_mut(&self) -> RwLockWriteGuard<EnvBase> {
        self.0.write().unwrap_or_else(|e| e.into_inner())
    }
}


#[derive(Debug, Clone)]
pub(crate) struct EnvBase {
    pub default_endian: Endian,
}
