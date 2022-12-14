class Solution {
public:
    int count(vector<int>& nums, int v) {
        int ret = 0;
        for(auto x: nums) {
            if(v==x) {
                ret++;
            }
        }
        return ret;
    }

    // if x has at least than n/(k+1) occurences, x will be guaranteed in the result
    // output array is NOT guaranteed has k elements with highest occurences
    vector<int> vote(vector<int>& nums, int k = 2) {
        vector<int> candidates(k, 1e9+1);
        vector<int> scores(k, 0);
        for(auto x: nums) {
            bool matched = false;
            for(int i = 0;i<k;i++) {
                if(candidates[i] == x) {
                    scores[i]++;
                    matched = true;
                    break;
                }
            }
            for(int i = 0;!matched && i<k;i++) {
                if(scores[i] == 0) {
                    candidates[i] = x;
                    scores[i] = 1;
                    matched = true;
                    break;
                }
            }
            if(!matched) {
                for(auto& score: scores) {
                    score--;
                }
            }
        }
        return candidates;
    }

    vector<int> majorityElement(vector<int>& nums) {
        const int n = 3;
        auto candidates = vote(nums, n-1);
        vector<int> ret;
        auto threshold = nums.size()/n;
        for(auto c: candidates) {
            if(count(nums, c) > threshold) {
                ret.push_back(c);
            }
        }
        return ret;
    }
};