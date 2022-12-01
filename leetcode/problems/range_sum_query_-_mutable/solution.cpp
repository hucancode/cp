class NumArray {
public:
    vector<int> data;
    NumArray(vector<int>& nums) {
        int n = nums.size();
        data.resize(n, 0);
        for(int i = 0;i<n;i++) {
            add(i, nums[i]);
        }
    }
    int goUp(int i) {
        // return the closest element that covers element i
        return i | (i+1);
    }
    int goBack(int i) {
        // return the closest element behind i
        // i will covers [j, i], j = i&(i+1), so the next element is j-1;
        int j = i & (i+1);
        return j-1;
    }
    void add(int i, int delta) {
        int n = data.size();
        while(i < n) {
            data[i] += delta;
            i = goUp(i);
        }
    }
    void update(int index, int val) {
        int delta = val - sumRange(index, index);
        add(index, delta);
    }
    int sum(int right) {
        int ret = 0;
        while(right >= 0) {
            ret += data[right];
            right = goBack(right);
        }
        return ret;
    }
    int sumRange(int left, int right) {
        if(left == 0) {
            return sum(right);
        }
        return sum(right) - sum(left-1);
    }
};

/**
 * Your NumArray object will be instantiated and called as such:
 * NumArray* obj = new NumArray(nums);
 * obj->update(index,val);
 * int param_2 = obj->sumRange(left,right);
 */