struct ParkingSystem {
    slots: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {

    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {slots: vec![0, big, medium, small]}
    }
    
    fn add_car(&mut self, car_type: i32) -> bool {
        if self.slots[car_type as usize] <= 0 {
            false
        } else {
            self.slots[car_type as usize] -= 1;
            true
        }
    }
}

/**
 * Your ParkingSystem object will be instantiated and called as such:
 * let obj = ParkingSystem::new(big, medium, small);
 * let ret_1: bool = obj.add_car(carType);
 */