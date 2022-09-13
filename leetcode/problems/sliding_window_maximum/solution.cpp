class Solution {
public:
    void print(vector<int>& nums, deque<int>& window) {
        cout<<"window: ";
        for(const auto&x: window) {
            cout<<nums[x]<<" ";
        }
        cout<<endl;
    }
    vector<int> maxSlidingWindow(vector<int>& nums, int k) {
        int n = nums.size();
        deque<int> window;
        vector<int> ret;
        ret.reserve(n-k+1);
        int i = 0;
        for(;i<k;i++) {
            //cout<<"check "<<nums[i]<<endl;
            while(!window.empty() && nums[window.back()] <= nums[i]) {
                window.pop_back();
            }
            window.push_back(i);
        }
        ret.push_back(nums[window.front()]);
        //print(nums, window);
        for(;i<n;i++) {
            //cout<<"check "<<nums[i]<<endl;
            while(!window.empty() && nums[window.back()] <= nums[i]) {
                window.pop_back();
            }
            window.push_back(i);
            while(!window.empty() && i - window.front() >= k) {
                window.pop_front();
            }
            //print(nums, window);
            ret.push_back(nums[window.front()]);
        }
        return ret;
    }
};