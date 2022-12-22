/// Number of bits used to represent a block index value in the PoolAllocator
const BLOCK_BITS: u32 = 13;
/// Mask used to retrieves a block index value from a [Slot]
const BLOCK_MASK: u32 = ((1 << BLOCK_BITS) - 1) << OFFSET_BITS;

/// Number of bits used to represent a offset value in the PoolAllocator block
/// The offset is expressed in number of required minimum alignment of [Entry]
const OFFSET_BITS: u32 = 16;
/// Mask used to retrieves a offset value from a [Slot]
const OFFSET_MASK: u32 = (1 << OFFSET_BITS) - 1;

/// Number of bits used to represent a probe value used in the PoolAllocator block to retrieve the correct [Entry]
const PROBE_HASH_BITS: u32 = 3;
/// Mask used to retrieves a probe value from a [Slot]
const PROBE_HASH_MASK: u32 = ((1 << PROBE_HASH_BITS) - 1) << (OFFSET_BITS + BLOCK_BITS);


/// Ensure that BLOCK_BITS + OFFSET_BITS + PROBE_HASH_BITS feet in 32 bits
const _: () = assert!(
    (BLOCK_BITS + OFFSET_BITS + PROBE_HASH_BITS) == u32::BITS,
    "Block + Offset + Probe BITS should be 32 bits"
);

/// Ensure that BLOCK_MASK + OFFSET_MASK + PROBE_HASH_MASK feet in 32 bits
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
