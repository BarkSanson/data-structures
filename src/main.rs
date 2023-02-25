use hashmap::HashMap;

pub mod hashmap;
pub mod linkedlist;

fn main() {
    let map: HashMap<u32, u32> = HashMap::new();
    let map2: HashMap<u32, u32> = HashMap::with_capacity(32);
    let map3: HashMap<u32, u32> = HashMap::with_load_factor(0.8);
}
