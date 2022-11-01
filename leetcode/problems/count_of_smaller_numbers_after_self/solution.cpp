typedef pair<int, int> pi;
typedef vector<pi> vpi;
typedef vector<int> vi;
class Solution {
public:
    void mergeSort(vpi::iterator l, vpi::iterator r, vi& ans) {
        auto d = distance(l, r);
        //cout<<"merge sort l = "<<(*l).first<<" d = "<<d<<endl;
        auto m = l+d/2;
        if(d > 1) {
            mergeSort(l, m, ans);
            mergeSort(m, r, ans);
        }
        // now range [l,m) is sorted, [m,r) is also sorted
        // everything in [l,m) has index < everything in [m, r)
        // we can easily update the answer for range [l, m)
        auto i = l, j = m;
        while(i<m) {
            while(j < r && i->first > j->first) {
                j++;
            }
            auto idx = i->second;
            ans[idx] += distance(m,j);
            i++;
        }
        // if [l,m) and [m,r) both sorted, inplace_merge make [l, r) sorted
        inplace_merge(l, m, r);
    }
    vector<int> countSmaller(vector<int>& nums) {
        int n = nums.size();
        vpi a;
        a.reserve(n);
        for(int i = 0;i<n;i++) {
            a.emplace_back(nums[i],i);
        }
        vi ans(n, 0);
        mergeSort(a.begin(), a.end(), ans);
        return ans;
    }
};