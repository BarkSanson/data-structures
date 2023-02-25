
const DEFAULT_INITIAL_CAPACITY: u32 = 16;
const DEFAULT_LOAD_FACTOR: f64 = 0.75;

pub struct HashMap<K, V> {
    buckets: Box<[Entry<K, V>]>,
    /// Maximum capacity of the current buckets array
    capacity: u32,

    /// Load factor of the table. The load factor is the maximum
    /// percentage of the buckets that can be used before there is
    /// an internal restructuring
    load_factor: f64,

    /// Current size of the HashMap
    size: u32,

    /// Next size value at which to resize
    threshold: u32
}

struct Entry<K, V> {
    hash: u32,
    key: K,
    value: V,
}

impl <K, V> HashMap<K, V> {
    pub fn new() -> Self {
        let capacity = DEFAULT_INITIAL_CAPACITY;
        let load_factor = DEFAULT_LOAD_FACTOR;
        let threshold = ((capacity as f64) * load_factor).round() as u32;

        HashMap { 
            buckets: Box::new([]), 
            capacity, 
            load_factor, 
            size: 0, 
            threshold,
        }
    }

    pub fn with_capacity(initial_capacity: u32) -> Self {
        let capacity = initial_capacity;
        let load_factor = DEFAULT_LOAD_FACTOR;
        let threshold = ((capacity as f64) * load_factor).round() as u32;

        HashMap {
            buckets: Box::new([]), 
            capacity,
            load_factor,
            threshold,
            size: 0
        }
    }

    pub fn with_load_factor(load_factor: f64) -> Self {
        let capacity = DEFAULT_INITIAL_CAPACITY;
        let load_factor = load_factor;
        let threshold = ((capacity as f64) * load_factor).round() as u32;

        HashMap {
            buckets: Box::new([]), 
            capacity,
            load_factor,
            threshold,
            size: 0
        }
    }

}