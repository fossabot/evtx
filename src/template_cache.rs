use crate::binxml::tokens::read_template_definition;

use crate::model::deserialized::BinXMLTemplateDefinitionData;
use crate::Offset;
pub use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::io::{Cursor, Seek, SeekFrom};
use std::rc::Rc;
use crate::evtx_chunk::EvtxChunk;
use std::cell::{RefCell, Ref};
use core::borrow::Borrow;

pub type CachedTemplate<'chunk> = BinXMLTemplateDefinitionData<'chunk>;

#[derive(Debug, Default)]
pub struct TemplateCache<'chunk>(pub RefCell<HashMap<Offset, CachedTemplate<'chunk>>>);

impl<'chunk> TemplateCache<'chunk> {
    pub fn new() -> Self {
        TemplateCache(RefCell::new(HashMap::new()))
    }

    pub fn populate(data: &'chunk [u8], offsets: &[Offset]) -> Result<Self, failure::Error> {
        let mut cache = HashMap::new();
        let mut cursor = Cursor::new(data);

        for offset in offsets.iter().filter(|&&offset| offset > 0) {
            cursor.seek(SeekFrom::Start(u64::from(*offset)))?;
            let definition = read_template_definition(&mut cursor, None)?;
            cache.insert(*offset, definition);
        }

        Ok(TemplateCache(RefCell::new(cache)))
    }

    pub fn get_template_data_offset<'a: 'chunk>(&'a self, offset: Offset) -> Option<u32> {
        self.0.borrow().get(&offset).map(|template_def| template_def.data_size)
    }

    pub fn put_template(&self, offset: Offset, template: CachedTemplate<'chunk>) {
        self.0.borrow_mut().insert(offset, template);
    }


    pub fn get_template_cache<'a: 'chunk>(&'a self) -> Ref<HashMap<Offset, CachedTemplate<'chunk>>> {
        self.0.borrow()
    }

    pub fn len(&self) -> usize {
        self.0.borrow().len()
    }
}
