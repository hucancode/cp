class Solution {
public:
    int hardestWorker(int n, vector<vector<int>>& logs) {
        vector<int> f(n, 0);
        int time = 0;
        int ret = 0;
        int longest = 0;
        for(const auto& task: logs) {
            auto id = task[0];
            auto leave = task[1];
            auto d = leave - time;
            if(d > longest || d == longest && id < ret) {
                longest = d;
                ret = id;
            }
            time = leave;
        }
        return ret;
    }
};