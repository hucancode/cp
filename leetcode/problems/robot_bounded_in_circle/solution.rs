impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut dx = 0;
        let mut dy = 0;
        let mut dr = 0;
        for c in instructions.chars() {
            match c {
                'L' => dr = (dr+3)%4,
                'R' => dr = (dr+1)%4,
                _ => {
                    match dr {
                        0 => dy += 1,
                        1 => dx += 1,
                        2 => dy -= 1,
                        _ => dx -= 1,
                    }
                },
            }
        }
        let no_translation = dx == 0 && dy == 0;
        let has_rotation = dr != 0;
        no_translation || has_rotation
    }
}