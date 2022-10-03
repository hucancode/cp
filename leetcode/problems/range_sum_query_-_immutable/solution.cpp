class NumArray {
public:
    vector<int> data;
    NumArray(vector<int>& nums) {
        int n = nums.size();
        data.resize(n+1, 0);
        for (int i = 0; i < n; i++) {
            add(i, nums[i]);
        }
    }
    int getParent(int i) const {
        return i - (i & (-i));
    }

    int getNext(int i) const {
        return i + (i & (-i));
    }
    
    int getSum(int i) const {
        int sum = 0;
        ++i;
        while (i > 0) {
            sum += data[i];
            i = getParent(i);
        }
        return sum;
    }
    
    int sumRange(int i, int j) const {
        return getSum(j) - getSum(i-1);
    }
    
    void add(int i, int v) {
        ++i;
        while (i < data.size()) {
            data[i] += v;
            i = getNext(i);
        }
    }
};

/**
 * Your NumArray object will be instantiated and called as such:
 * NumArray* obj = new NumArray(nums);
 * int param_1 = obj->sumRange(left,right);
 */