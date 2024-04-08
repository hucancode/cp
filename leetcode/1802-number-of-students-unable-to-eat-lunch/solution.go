func countStudents(students []int, sandwiches []int) int {
    a := 0
    for _, x := range students {
        a += x
    }
    b := len(students) - a
    for _, x := range sandwiches {
        if(x == 1 && a > 0) {
            a--
        } else if(x == 0 && b > 0) {
            b--
        } else {
            break
        }
    }
    return a + b
}
