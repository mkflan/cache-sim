#![allow(unused)]

use rand::Rng;
use std::{fmt, ops::Deref};

// The number of sets of cache lines in the cache.
const CACHE_SETS: usize = 4;

// The length of a cache line, in bytes.
const CACHE_LINE_LEN: usize = std::mem::size_of::<u128>();

// The number of offset bits in a memory address.
const OFFSET_BITS: u32 = CACHE_LINE_LEN.ilog2();

// The number of index bits.
const INDEX_BITS: u32 = CACHE_SETS.ilog2();

// The number of tag bits.
const TAG_BITS: u32 = 32 - OFFSET_BITS - INDEX_BITS;

// A 32-bit memory address.
//
// It is organized into, starting from the least significant bit:
//  - log2(CACHE_LINE_LEN) = offset bits.
//  - log2(CACHE_SETS) index bits.
//  - Tag bits, which take up the remaining bits.
#[derive(Debug, Clone, Copy)]
struct MemoryAddress(u32);

impl MemoryAddress {
    fn extract_offset(self) -> u32 {
        let mask = (1 << OFFSET_BITS) - 1;
        self.0 & mask
    }

    fn extract_index(self) -> u32 {
        let mask = (1 << INDEX_BITS) - 1;
        (self.0 >> INDEX_BITS) & mask
    }

    fn extract_tag(self) -> u32 {
        let mask = (1 << (TAG_BITS)) - 1;
        (self.0 >> (OFFSET_BITS + INDEX_BITS)) & mask
    }
}

impl fmt::UpperHex for MemoryAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self.0)
    }
}

// A cache line.
#[derive(Default, Clone, Copy, Debug)]
struct CacheLine {
    valid: bool,
    tag: u32,
    data: u128, // 16 bytes of data
}

// Currently, the cache is implemented as a direct-mapped cache where each memory address refers to one cache line.
// In the future, the functionality will be extended to support simulating other cache organization strategies such
// as set-associative caches.
#[derive(Default, Clone, Copy, Debug)]
struct Cache([CacheLine; CACHE_SETS]);

impl Cache {
    fn new() -> Self {
        Self::default()
    }

    fn extract_line(self, idx: u32) -> CacheLine {
        self.0[idx as usize]
    }

    // Compare the address tag and tag of corresponding cache line.
    fn compare_tags(self, idx: u32, addr_tag: u32) -> bool {
        addr_tag == self.extract_line(idx).tag
    }

    fn search_cache(self, addr: MemoryAddress) -> Option<CacheLine> {
        let idx = addr.extract_index();

        if self.compare_tags(idx, addr.extract_tag()) {
            return Some(self.extract_line(idx));
        }

        None
    }
}

fn main() {
    let mut cache = Cache::new();
    let mut rng = rand::rng();

    // println!("Number of Cache Sets:      {}", CACHE_SETS);
    // println!("Byte Length of Cache Line: {} bytes", CACHE_LINE_LEN);
    // println!("Number of Offset Bits:     {}", OFFSET_BITS);
    // println!("Number of Index Bits:      {}", INDEX_BITS);

    // let addrs: Vec<MemoryAddress> = (0..16).map(|_| MemoryAddress(rng.next_u32())).collect();

    // for (idx, addr) in addrs.into_iter().enumerate() {
    //     println!("Address {}", idx + 1);
    //     println!("  Hex:    {:X}", addr);
    //     println!("  Binary: {:b}", addr.0);
    //     println!("    Offset Bits: {:b}", addr.extract_offset());
    //     println!("    Index Bits:  {:b}", addr.extract_index());
    //     println!("    Tag Bits:    {:b}", addr.extract_tag());
    // }
}
