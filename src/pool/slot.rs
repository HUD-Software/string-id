const BLOCK_BITS: u32 = 13;
const BLOCK_MASK: u32 = ((1 << BLOCK_BITS) - 1) << OFFSET_BITS;

const OFFSET_BITS: u32 = 16;
const OFFSET_MASK: u32 = (1 << OFFSET_BITS) - 1;

const PROBE_HASH_BITS: u32 = 3;
const PROBE_HASH_MASK: u32 = ((1 << PROBE_HASH_BITS) - 1) << (OFFSET_BITS + BLOCK_BITS);

const _: () = assert!(
    (BLOCK_BITS + OFFSET_BITS + PROBE_HASH_BITS) == u32::BITS,
    "Block + Offset + Probe BITS should be 32 bits"
);
const _: () = assert!(
    (BLOCK_MASK + OFFSET_MASK + PROBE_HASH_MASK) == u32::MAX,
    "Block + Offset + Probe MASK be 32 bits"
);

pub struct Slot(u32);

impl Slot {
    pub fn offset(&self) -> u32 {
        self.0 & OFFSET_MASK
    }

    pub fn block(&self) -> u32 {
        (self.0 & BLOCK_MASK) >> OFFSET_BITS
    }

    pub fn probe_hash(&self) -> u32 {
        (self.0 & PROBE_HASH_MASK) >> OFFSET_BITS + BLOCK_BITS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slot_offset() {
        let slot = Slot(OFFSET_MASK);
        assert_eq!(slot.offset(), OFFSET_MASK);
    }

    fn slot_block() {
        let slot = Slot(BLOCK_MASK);
        assert_eq!(slot.block(), BLOCK_MASK)
    }

    fn slot_probe_hash() {
        let slot = Slot(PROBE_HASH_MASK);
        assert_ne!(slot.probe_hash(), PROBE_HASH_MASK);
    }
}