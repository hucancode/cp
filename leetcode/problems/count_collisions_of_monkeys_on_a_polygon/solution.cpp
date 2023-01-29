#define INF ((long long)(1e9+7LL))
class Solution {
public:
    long long fastPow(int x, int n) {
        if(n == 0) {
            return 1;
        }
        auto k = fastPow(x, n/2);
        k = k*k%INF;
        cout<<"k = "<<k<<" n = "<<n<<endl;
        if(n%2 == 0) {
            return k;
        }
        return k*x%INF;
    }
    int monkeyMove(int n) {
        return (fastPow(2, n)+INF - 2)%INF;
    }
};