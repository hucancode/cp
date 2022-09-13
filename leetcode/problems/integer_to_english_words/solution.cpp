class Solution {
public:
    const string zero0[10] = {
      "",
      "One",
      "Two",
      "Three",
      "Four",
      "Five",
      "Six",
      "Seven",
      "Eight",
      "Nine"
    };
    const string zero1[10] = {
      "Ten",
      "Eleven",
      "Twelve",
      "Thirteen",
      "Fourteen",
      "Fifteen",
      "Sixteen",
      "Seventeen",
      "Eighteen",
      "Nineteen"
    };

    const string zero2[11] = {
      "",
      "",
      "Twenty",
      "Thirty",
      "Forty",
      "Fifty",
      "Sixty",
      "Seventy",
      "Eighty",
      "Ninety"
    };

    const string zero3x[5] = {
      "Hundred",
      "Thousand",
      "Million",
      "Billion",
      "Trillion"
    };

    const long factor[5] = {
      100L,
      1000L,
      1000000L,
      1000000000L,
      1000000000000L
    };
    string trimr(string s) {
        return s.erase(s.find_last_not_of(' ') + 1);
    }
    string solve(int n) {
        for(int i = 4;i>=0;i--) {
            if(n >= factor[i]) {
                return trimr(solve(n/factor[i]) + " " + zero3x[i] + " " + solve(n%factor[i]));
            }
        }
        if(n >= 20) {
            return trimr(zero2[n/10] + " " + solve(n%10));
        }
        if(n >= 10) {
            return zero1[n-10];
        }
        return zero0[n];
    }
    string numberToWords(int n) {
        if(n == 0) {
            return "Zero";
        }
        return solve(n);
    }
};