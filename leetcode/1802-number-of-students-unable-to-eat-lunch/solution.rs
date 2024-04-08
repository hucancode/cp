impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut a: i32 = students.iter().sum();
        let mut b: i32 = students.len() as i32 - a;
        for x in sandwiches {
            if x == 1 && a > 0 {
                a -= 1;
            } else if x == 0 && b > 0 {
                b -= 1;
            } else {
                break;
            }
        }
        a + b
    }
}
