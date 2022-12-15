class Solution {
    vector<int> arr;
    vector<int> prefix;
    vector<int> nonZeroCount;
public:
    int sum(int left, int right) {
        return prefix[right] - prefix[left];
    }
    // split(begin, end, k, best, worst)
    // find a way to split [begin, end] to k section
    // the best result to this point is now 'best', there is no point to split smaller than this
    // the worst result to this point is 'worst', if we have to split a section bigger than this, it's better to stop and expand the previous section
    int split(int begin, int end, int k, int best, int worst) {
        if(nonZeroCount[end] - nonZeroCount[begin] <= k) {
            return *max_element(arr.begin()+begin, arr.begin()+end);
        }
        if(k == 1) {
            return sum(begin, end);
        }
        int i = distance(prefix.begin(), 
            upper_bound(prefix.begin()+begin, prefix.begin()+end, prefix[begin]+best)) - 1;
        i = max(i, begin+1);
        int j = distance(prefix.begin(), 
            upper_bound(prefix.begin()+i, prefix.begin()+end, prefix[begin]+worst));
        j = min(j, end-k+1);
        i = min(i, j);
        int ret = 2e9;
        for(;i<=j;i++) {
            auto a = sum(begin, i);
            if(a >= ret) {
                continue;
            }
            auto nextA = sum(begin, i+1);
            auto b = split(i, end, k-1, max(a, best), min(nextA, worst));
            auto score = max(a, b);
            ret = min(ret, score);
        }
        //cout<<"split "<<begin<<"-"<<end<<" "<<k<<" returns "<<ret<<endl;
        return ret;
    }
    int splitArray(vector<int>& nums, int k) {
        arr = move(nums);
        int n = arr.size();
        prefix.resize(n+1,0);
        nonZeroCount.resize(n+1,0);
        for(int i = 1;i<=n;i++) {
            prefix[i] = prefix[i-1] + arr[i-1];
            nonZeroCount[i] = nonZeroCount[i-1]+(arr[i-1] != 0);
        }
        return split(0, n, k, 0, prefix[n-k]);
    }
};