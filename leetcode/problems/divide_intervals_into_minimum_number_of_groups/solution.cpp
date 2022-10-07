class Solution {
public:
    int minGroups(vector<vector<int>>& intervals) {
        auto cmp = [](const vector<int>& a, const vector<int>& b) {
            return a[0] < b[0];
        };
		sort(intervals.begin(), intervals.end(), cmp);
        int ret = 0;
		priority_queue<int, vector<int>, greater<int> > tails;
		for(const auto& interval: intervals) {
            //cout<<"check range "<<interval[0]<<"-"<<interval[1]<<endl;
            while(!tails.empty() && interval[0] > tails.top()) {
                tails.pop();
            }
            auto tail = interval[1];
			tails.push(tail);
			ret = max(ret, (int)tails.size());
		}
        return ret;
    }
};
