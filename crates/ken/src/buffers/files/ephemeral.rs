use pecia::Buffer;
use slotmap::{SlotMap, new_key_type};

new_key_type! { pub struct EphemeralKey; }

pub struct Ephemeral(SlotMap<EphemeralKey, Buffer>);

impl Ephemeral {
    pub fn new() -> Self {
        Self(SlotMap::with_key())
    }
    
    pub fn get(
        &self,
        key: EphemeralKey
    ) -> Option<&Buffer> {
        self.0.get(key)
    }
    
    pub fn insert(&mut self) -> EphemeralKey {
        self.0.insert(Buffer::vector())
    }
    
    pub fn remove(
        &mut self,
        key: EphemeralKey
    ) {
        self.0.remove(key);
    }
}