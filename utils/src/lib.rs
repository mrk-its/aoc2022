#![no_std]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(core_intrinsics)]
// #![feature(nll)]

pub mod hash;
use core::panic::PanicInfo;
use ufmt_stdio::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("PANIC!!!");
    core::intrinsics::abort();
}

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[start]
        fn _main(_argc: isize, _argv: *const *const u8) -> isize {
            // type check the given path
            let f: fn() -> () = $path;
            f();

            #[cfg(target_vendor = "a800xl")]
            loop {}

            #[cfg(not(target_vendor = "a800xl"))]
            0
        }
    };
}

#[macro_export]
macro_rules! iter_lines {
    ($name:expr) => {
        include_bytes!($name).split(|c| *c == b'\n')
    };
}

pub fn to_str(data: &[u8]) -> &str {
    unsafe { core::str::from_utf8_unchecked(data) }
}

pub trait SimpleHash {
    fn hash(&self) -> usize {
        0
    }
}

impl SimpleHash for &[u8] {
    fn hash(&self) -> usize {
        let mut h = 0;
        for c in self.iter() {
            h = hash::hash8(h, *c);
        }
        h as usize
    }
}

impl SimpleHash for u8 {
    fn hash(&self) -> usize {
        *self as usize
    }
}

pub struct BitSet<const N: usize>
where
    [(); (N + 7) / 8]:,
{
    bits: [u8; (N + 7) / 8],
}

impl<const N: usize> BitSet<N>
where
    [(); (N + 7) / 8]:,
{
    pub fn new() -> Self {
        Self {
            bits: [0; (N + 7) / 8],
        }
    }
    pub fn contains(&self, index: usize) -> bool {
        let offs = index / 8;
        let bit_offs = index & 7;
        return (self.bits[offs] >> bit_offs) & 1 > 0;
    }
    pub fn insert(&mut self, index: usize) {
        let offs = index / 8;
        let bit_offs = index & 7;
        self.bits[offs] |= 1 << bit_offs;
    }
    pub fn intersect(&mut self, other: &BitSet<N>) {
        for (a, b) in self.bits.iter_mut().zip(other.bits.iter()) {
            *a &= *b;
        }
    }
}

pub struct SimpleMap<const N: usize, K, V>
where
    K: Eq + SimpleHash,
{
    data: [Option<(K, V)>; N],
}

impl<const N: usize, K, V> SimpleMap<N, K, V>
where
    K: Eq + SimpleHash,
{
    const INIT: Option<(K, V)> = None;
    pub fn new() -> Self {
        Self {
            data: [Self::INIT; N],
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let mut index = key.hash() % N;
        let start_index = index;
        loop {
            match &self.data[index] {
                Some((k, _)) => {
                    if key == *k {
                        self.data[index] = Some((key, value));
                        return;
                    }
                }
                None => {
                    self.data[index] = Some((key, value));
                    return;
                }
            }
            index = (index + 1) % N;
            if index == start_index {
                panic!("no free space");
            }
        }
    }
    pub fn contains(&self, key: &K) -> bool {
        let mut index = key.hash() % N;
        let start_index = index;
        loop {
            match &self.data[index] {
                Some((k, _)) => {
                    if *key == *k {
                        return true;
                    }
                }
                None => {
                    return false;
                }
            }
            index = (index + 1) % N;
            if index == start_index {
                return false;
            }
        }
    }
    pub fn entry<'a>(&'a mut self, key: K) -> Entry<'a, N, K, V> {
        let mut index = key.hash() % N;
        let start_index = index;
        loop {
            match &self.data[index] {
                Some((k, _)) => {
                    if key == *k {
                        return Entry::Occupied(OccupiedEntry {
                            key,
                            map: self,
                            index,
                        });
                    }
                }
                None => {
                    return Entry::Vacant(VacantEntry {
                        key,
                        map: self,
                        index,
                    })
                }
            }
            index = (index + 1) % N;
            if index == start_index {
                panic!("no free space");
            }
        }
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        let mut index = key.hash() % N;
        let start_index = index;
        loop {
            match &self.data[index] {
                Some((k, v)) => {
                    if *key == *k {
                        return Some(v);
                    }
                }
                None => {
                    return None;
                }
            }
            index = (index + 1) % N;
            if index == start_index {
                return None;
            }
        }
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.data
            .iter()
            .filter(|k| k.is_some())
            .map(|k| &k.as_ref().unwrap().1)
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.data
            .iter()
            .filter(|k| k.is_some())
            .map(|k| &k.as_ref().unwrap().0)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.data
            .iter()
            .filter(|k| k.is_some())
            .map(|k| k.as_ref().unwrap())
            .map(|k| (&k.0, &k.1))
    }
}
pub struct SimpleSet<const N: usize, K>(SimpleMap<N, K, ()>)
where
    K: Eq + SimpleHash;

impl<const N: usize, K> SimpleSet<N, K>
where
    K: Eq + SimpleHash,
{
    pub fn new() -> Self {
        Self(SimpleMap::new())
    }
    pub fn insert(&mut self, key: K) {
        self.0.insert(key, ());
    }
    pub fn contains(&self, key: &K) -> bool {
        self.0.contains(key)
    }
    pub fn iter(&self) -> impl Iterator<Item = &K> {
        self.0.keys()
    }
}
pub enum Entry<'a, const N: usize, K: 'a, V: 'a>
where
    K: Eq + SimpleHash,
{
    Occupied(OccupiedEntry<'a, N, K, V>),
    Vacant(VacantEntry<'a, N, K, V>),
}

impl<'a, const N: usize, K: 'a, V: 'a> Entry<'a, N, K, V>
where
    K: Eq + SimpleHash,
{
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => &mut entry.map.data[entry.index].as_mut().unwrap().1,
            Entry::Vacant(entry) => entry.insert(default),
        }
    }
}

impl<'a, const N: usize, K: 'a, V: 'a> Entry<'a, N, K, V>
where
    K: Eq + SimpleHash,
    V: Default,
{
    pub fn or_default(self) -> &'a mut V {
        self.or_insert(Default::default())
    }
}

pub struct OccupiedEntry<'a, const N: usize, K: 'a, V: 'a>
where
    K: Eq + SimpleHash,
{
    key: K,
    index: usize,
    map: &'a mut SimpleMap<N, K, V>,
}

impl<'a, const N: usize, K: 'a, V: 'a> OccupiedEntry<'a, N, K, V>
where
    K: Eq + SimpleHash,
{
    pub fn key(&self) -> &K {
        &self.key
    }
    pub fn get(&self) -> &V {
        &self.map.data[self.index].as_ref().unwrap().1
    }
    pub fn get_mut(&mut self) -> &mut V {
        &mut self.map.data[self.index].as_mut().unwrap().1
    }
}

pub struct VacantEntry<'a, const N: usize, K: 'a, V: 'a>
where
    K: Eq + SimpleHash,
{
    key: K,
    index: usize,
    map: &'a mut SimpleMap<N, K, V>,
}

impl<'a, const N: usize, K: 'a, V: 'a> VacantEntry<'a, N, K, V>
where
    K: Eq + SimpleHash,
{
    pub fn key(&self) -> &K {
        &self.key
    }
    pub fn insert(self, value: V) -> &'a mut V {
        self.map.data[self.index] = Some((self.key, value));
        &mut self.map.data[self.index].as_mut().unwrap().1
    }
}
