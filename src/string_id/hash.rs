use cityhash_sys::city_hash_64;
use core::convert::From;

const PoolShardBits: u32 = 10;
const PoolShardCount: u32 = 1 << PoolShardBits;
const ShardMask: u32 = PoolShardCount - 1;

pub struct Hash(u64);

impl Hash {
    pub fn shard_index(&self) -> u32 {
        (self.0 >> 32) as u32 & ShardMask
    }
    pub fn probe_start_index(&self) -> u32 {
        self.0 as u32
    }
}

impl From<&str> for Hash {
    fn from(string: &str) -> Self {
        Hash(city_hash_64(string.as_bytes()))
    }
}

// pub struct StringIdHash {
//     pub shard_index: u32,
//     pub probe_start_index: u32
// }

// impl From<&str> for StringIdHash {
//     fn from(string: &str) -> Self {
//         let hash = Hash::from(string);
//         StringIdHash {
//             shard_index: hash.shard_index(),
//             probe_start_index: hash.probe_start_index(),
//         }
//     }
// }
