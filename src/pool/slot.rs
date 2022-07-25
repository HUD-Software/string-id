#[derive(PartialEq, Eq)]
struct Offset(u32);

impl Offset {
    pub const BITS: u32 = 16;
    pub const MASK: u32 = (1 << Offset::BITS) - 1;
}

#[derive(PartialEq, Eq)]
struct Block(u32);

impl Block {
    pub const BITS: u32 = 13;
    pub const MASK: u32 = ((1 << Block::BITS) - 1) << Offset::BITS;
}

#[derive(PartialEq, Eq)]
struct ProbeHash(u32);

impl ProbeHash {
    pub const BITS: u32 = 3;
    pub const MASK: u32 = ((1 << ProbeHash::BITS) - 1) << (Offset::BITS + Block::BITS);
}

const _: () = assert!(
    (Block::BITS + Offset::BITS + ProbeHash::BITS) == u32::BITS,
    "Block + Offset + Probe BITS should be 32 bits"
);
const _: () = assert!(
    (Block::MASK + Offset::MASK + ProbeHash::MASK) == u32::MAX,
    "Block + Offset + Probe MASK be 32 bits"
);

pub struct Slot(u32);

impl Slot {
    pub fn offset(&self) -> Offset {
        Offset(self.0 & Offset::MASK)
    }

    pub fn block(&self) -> Block {
        Block((self.0 & Block::MASK) >> Offset::BITS)
    }

    pub fn probe_hash(&self) -> ProbeHash {
        ProbeHash((self.0 & ProbeHash::MASK) >> Offset::BITS + Block::BITS)
    }
}

#[cfg(test)]
mod tests {
    use super::{Block, Offset, ProbeHash, Slot};

    fn slot_offset() {
        let slot = Slot(Offset::MASK);
        assert_eq!(slot.offset().0, Offset::MASK);
    }

    fn slot_block() {
        let slot = Slot(Block::MASK);
        assert_eq!(slot.block().0, Block::MASK)
    }

    fn slot_probe_hash() {
        let slot = Slot(ProbeHash::MASK);
        assert_ne!(slot.probe_hash().0, ProbeHash::MASK);
    }
}
