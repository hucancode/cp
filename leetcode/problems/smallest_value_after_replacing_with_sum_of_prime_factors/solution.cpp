class Solution {
public:
    vector<int> primes;
    vector<bool> isPrime;
    void build() {
        int n = 1e5;
        isPrime.resize(n+1, true);
        isPrime[0] = isPrime[1] = false;
        primes.reserve(1e4);
        for (long long i = 2; i <= n; i++) {
            if (isPrime[i]) {
                for (long long j = i * i; j <= n; j += i) {
                    isPrime[j] = false;
                }
                primes.push_back(i);
            }
        }
    }
    vector<int> factors(int n) {
        vector<int> ret;
        while(n != 1) {
            for(auto x: primes) {
                if(x > n) {
                    break;
                }
                if(n%x == 0) {
                    ret.push_back(x);
                    n = n/x;
                }
            }
        }
        return ret;
    }
    int smallestValue(int n) {
        build();
        if(isPrime[n]) {
            return n;
        }
        auto f = factors(n);
        int next = 0;
        for(auto x: f) {
            next += x;
        }
        if(next >= n) {
            return n;
        }
        return smallestValue(next);
    }
};