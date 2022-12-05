class RangeFreqQuery {
    map<int, vector<int>> indices;
public:
    RangeFreqQuery(vector<int>& arr) {
        int n = arr.size();
        for(int i = 0;i<n;i++) {
            indices[arr[i]].push_back(i);
        }
    }
    
    int query(int left, int right, int value) {
        auto& pos = indices[value];
        auto i = lower_bound(pos.begin(), pos.end(), left);
        auto j = upper_bound(i, pos.end(), right);
        return distance(i,j);
    }
};

/**
 * Your RangeFreqQuery object will be instantiated and called as such:
 * RangeFreqQuery* obj = new RangeFreqQuery(arr);
 * int param_1 = obj->query(left,right,value);
 */