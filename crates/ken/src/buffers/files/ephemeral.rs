use landmine::Landmine;
use pecia::Buffer as Inner;
use slotmap::{SlotMap, new_key_type};

use crate::Buffer;

new_key_type! { pub struct EphemeralKey; }

pub struct Ephemeral(SlotMap<EphemeralKey, Buffer>);

impl Ephemeral {
    pub fn new() -> Self {
        Self(SlotMap::with_key())
    }
    
    pub fn get(
        &self,
        key: EphemeralKey
    ) -> Landmine<&Buffer> {
        self.0.get(key).into()
    }

    pub fn get_mut(
        &mut self,
        key: EphemeralKey
    ) -> Landmine<&mut Buffer> {
        self.0.get_mut(key).into()
    }
    
    pub fn insert(&mut self) -> EphemeralKey {
        self.0.insert(Buffer::new(Inner::vector()))
    }
    
    pub fn remove(
        &mut self,
        key: EphemeralKey
    ) {
        self.0.remove(key);
    }
}