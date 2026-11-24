use pecia::Buffer;
use slotmap::{SlotMap, new_key_type};

new_key_type! { pub struct DraftKey; }

pub struct Drafts(SlotMap<DraftKey, Buffer>);

impl Drafts {
    pub fn new() -> Self {
        Self(SlotMap::with_key())
    }
    
    pub fn get(
        &self,
        key: DraftKey
    ) -> Option<&Buffer> {
        self.0.get(key)
    }
    
    pub fn insert(&mut self) -> DraftKey {
        self.0.insert(Buffer::vector())
    }
    
    pub fn remove(
        &mut self,
        key: DraftKey
    ) {
        self.0.remove(key);
    }
}