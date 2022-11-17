class Solution {
public:
    int scheduleCourse(vector<vector<int>>& courses) {
        int n = courses.size();
        for(auto& course: courses) {
            swap(course[0], course[1]);
        }
        sort(courses.begin(), courses.end());
        int time = 0;
        priority_queue<pair<int,int>> q;// item = (duration, index)
        for(int i = 0;i<n;i++) {
            int duration = courses[i][1];
            int lastDay = courses[i][0];
            bool compatible = time + duration <= lastDay;
            if(compatible) {
                q.emplace(duration, i);
                time += duration;
                continue;
            }
            if(q.empty()) {
                continue;
            }
            int topDuration, topIdx;
            tie(topDuration, topIdx) = q.top();
            if(duration < topDuration) {
                q.pop();
                q.emplace(duration, i);
                time -= topDuration - duration;
            }
        }
        return q.size();
    }
};