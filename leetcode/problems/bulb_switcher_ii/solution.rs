impl Solution {
    pub fn flip_lights(n: i32, presses: i32) -> i32 {
        if presses == 0 {
            return 1;
        }
        if n == 1 {
            // press 1/3/4 yield 1 state
            // press 2 yield 1 state
            return 2;
        }
        if presses == 1 {
            if n == 2 {
                return 3;
            }
            return 4;
        }
        if n == 2 {
            // 11/22/33/44/34 yield 1 state
            // 12, 13/14, 23/24 each yield 1 state
            // from the 3rd press, no new state
            return 4;
        }
        if presses == 2 {
            // 11/22/33/44 yield 1 state
            // 12, 13, 14, 23, 24, 34 each yield 1 state
            return 7;
        }
        // xx1, xx2, xx3, xx4 each yield 1 state
        // 124, 134, 123, 234 each yield 1 state
        // from 4th press, no new state
        return 8;
    }
}