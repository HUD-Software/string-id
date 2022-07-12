const MAX_OFFSET_BITS: u32 = 16;
const MAX_BLOCK_BITS: u32 = 13;
const MAX_PROBE_BITS: u32 = 3;

const MAX_OFFSET_MASK: u32 = (1 << MAX_OFFSET_BITS) - 1;
const MAX_BLOCK_MASK: u32 = ((1 << MAX_BLOCK_BITS) - 1) << MAX_OFFSET_BITS;
const MAX_PROBE_MASK: u32 = ((1 << MAX_PROBE_BITS) - 1) << (MAX_OFFSET_BITS + MAX_BLOCK_BITS);

const A : u32 = MAX_OFFSET_MASK ^ MAX_BLOCK_MASK ^ MAX_PROBE_MASK;


#[derive(PartialEq, Eq)]
struct Offset(u32);

impl Offset{
    pub const BITS: u32 = 16;
    pub const MASK: u32 = (1 << Offset::BITS) - 1;
}

#[derive(PartialEq, Eq)]
struct Block(u32);

impl Block{
    pub const BITS: u32 = 13;
    pub const MASK: u32 = ((1 << Block::BITS) - 1) << Offset::BITS;
}

#[derive(PartialEq, Eq)]
struct ProbeHash(u32);

impl ProbeHash{
    pub const BITS: u32 = 3;
    pub const MASK: u32 = ((1 << ProbeHash::BITS) - 1) << (Offset::BITS + Block::BITS);
}


const _ : () = assert!((Block::BITS + Offset::BITS + ProbeHash::BITS) == u32::BITS, "Block + Offset + Probe BITS should be 32 bits");
const _ : () = assert!((Block::MASK + Offset::MASK + ProbeHash::MASK) == u32::MAX, "Block + Offset + Probe MASK be 32 bits");


struct Slot(u32);

impl Slot {
    fn offset(&self) -> Offset{
        Offset(self.0 & Offset::MASK)
    }

    fn block(&self) -> Block {
        Block((self.0 & Block::MASK) >> Offset::BITS)
    }

    fn probe_hash(&self) -> ProbeHash {
        ProbeHash((self.0 & ProbeHash::MASK) >> Offset::BITS+Block::BITS )
    }
}

#[cfg(test)]
mod tests {
    use super::{Slot, Offset, Block, ProbeHash};

    fn slot_offset() {
        let slot = Slot(Offset::MASK);
        assert_eq!(slot.offset().0, Offset::MASK);
    }

    fn slot_block(){
        let slot = Slot(Block::MASK);
        assert_eq!(slot.block().0, Block::MASK)
    }

    fn slot_probe_hash() {
        let slot = Slot(ProbeHash::MASK);
        assert_ne!(slot.probe_hash().0, ProbeHash::MASK);
    }
}