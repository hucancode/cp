class Solution {
public:
    int leastInterval(vector<char>& tasks, int n) {
        if(n == 0) {
            return tasks.size();
        }
        int ret = 0;
        map<char, int> counts;
        for(const auto& task: tasks) {
            if(counts.find(task) == counts.end()) {
                counts[task] = 0;
            }
            counts[task]++;
        }
        priority_queue<int> q;
        for(const auto& item: counts) {
            q.push(item.second);
        }
        int top = q.top();
        q.pop();
        int idle = (top - 1)*n;
        while(!q.empty() && idle > 0) {
            idle -= min(top - 1, q.top());
            q.pop();
        }
        ret = max(idle, 0) + tasks.size();
        return ret;
    }
};