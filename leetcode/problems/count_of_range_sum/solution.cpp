typedef long long ll;
typedef pair<ll, int> pi;
typedef vector<pi> vpi;
typedef vector<int> vi;
typedef vector<ll> vll;
class Solution {
    vi count; // count[i] = number of j appear before i that has lower <= prefix[i] - prefix[j] <= upper
public:
    void mergeSort(vpi::iterator l, vpi::iterator r, int lower, int upper) {
        auto d = distance(l, r);
        auto m = l + d/2;
        if(d > 1) {
            mergeSort(l, m, lower, upper);
            mergeSort(m, r, lower, upper);
        } else {
            // edge case
            auto value = l->first;
            auto idx = l->second;
            if(value <= upper && value >= lower) {
                count[idx]++;
            }
        }
        // now all items in range [l,m) has index less than [m,r) AND [l,m) sorted, [m,r) also sorted
        // for each item i on the right, find the number of item j on the left that satisfy:
        // lower <= prefix[i] - prefix[j] <= upper
        // prefix[i] - lower >= prefix[j] >= prefix[i] - upper
        for(auto it = m;it != r;it++) {
            auto value = it->first;
            auto idx = it->second;
            pi targetMax = {value - lower, 1e5+1};
            pi targetMin = {value - upper, 0};
            auto jmin = lower_bound(l, m, targetMin);
            auto jmax = upper_bound(jmin, m, targetMax);
            auto k = distance(jmin, jmax);
            count[idx] += k;
        }
        inplace_merge(l, m, r);
    }
    int countRangeSum(vector<int>& nums, int lower, int upper) {
        int n = nums.size();
        vll prefix(n);
        prefix[0] = nums[0];
        for(int i = 1;i<n;i++) {
            prefix[i] = prefix[i-1]+nums[i];
        }
        count.resize(n,0);
        vpi a;
        a.reserve(n);
        for(int i = 0;i<n;i++) {
            a.emplace_back(prefix[i], i);
        }
        mergeSort(a.begin(), a.end(), lower, upper);
        return accumulate(count.begin(), count.end(), 0);
    }
};