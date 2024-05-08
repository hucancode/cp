class Solution {
public:
    vector<string> findRelativeRanks(vector<int>& score) {
        auto n = score.size();
        vector<int> input(n);
        iota(input.begin(), input.end(), 0);
        sort(input.begin(), input.end(), [&](int a,int b) {
            return score[a] > score[b];
        });
        vector<string> medals = {
            "Gold Medal",
            "Silver Medal",
            "Bronze Medal",
        };
        vector<string> ret(n);
        for(int i = 0;i<n;i++) {
            ret[input[i]] = (i < 3)?medals[i]:to_string(i+1);
        }
        return ret;
    }
};
