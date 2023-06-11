use std::collections::HashMap;
struct SnapshotArray {
    data: Vec<Vec<(i32, i32)>>,
    buffer: HashMap<i32, i32>,
    snap_id: i32,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {

    fn new(length: i32) -> Self {
        let length = length as usize;
        Self {
            data: vec![vec![(-1, 0)]; length],
            buffer: HashMap::new(),
            snap_id: -1,
        }
    }
    
    fn set(&mut self, index: i32, val: i32) {
        self.buffer.insert(index,val);
    }
    
    fn snap(&mut self) -> i32 {
        self.snap_id += 1;
        for (&k,&v) in self.buffer.iter() {
            self.data[k as usize].push((self.snap_id, v));
        }
        self.buffer.clear();
        self.snap_id
    }
    
    fn get(&self, index: i32, snap_id: i32) -> i32 {
        //println!("find snap shot {snap_id}/{}", self.snap_id);
        let arr = &self.data[index as usize];
        let i = match arr.binary_search_by(|(i,_)| i.cmp(&snap_id)) {
            Ok(i) => i,
            Err(i) => i-1,
        };
        //println!("{arr:?}, found {i}");
        let (_,ret) = arr[i];
        ret
    }
}

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */