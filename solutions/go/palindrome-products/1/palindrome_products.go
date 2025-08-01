package palindrome
import (
    "errors"
)
type Product struct {
    Value int
    Factorizations [][2]int
}

func isPalindrome(x int) bool {
    digits := make([]int, 0)
    for x > 0 {
        digits = append(digits, x%10)
        x/=10
    }
    for i:=0;i<len(digits)/2;i++ {
        if digits[i] != digits[len(digits)-i-1] {
            return false
        }
    }
    return true
}
func Products(fmin, fmax int) (Product, Product, error) {
    a := Product{0, make([][2]int, 0)}
    b := Product{1000000000, make([][2]int, 0)}
    if fmin > fmax {
        return a, b, errors.New("fmin > fmax...")
    }
    for i:= fmin;i<=fmax;i++ {
        for j:=i;j<=fmax;j++ {
            x := i*j
            if isPalindrome(x) {
                if x > a.Value {
                    a.Value = x
                    a.Factorizations = [][2]int{{i, j}}
                } else if x == a.Value {
                    a.Factorizations = append(a.Factorizations, [2]int{i, j})
                }
                if x < b.Value {
                    b.Value = x
                    b.Factorizations = [][2]int{{i, j}}
                } else if x == b.Value {
                    a.Factorizations = append(a.Factorizations, [2]int{i, j})
                }
            }
        }
    }
    if a.Value == 0 {
        return a, b, errors.New("no palindromes...")
    }
    return b, a, nil
}
