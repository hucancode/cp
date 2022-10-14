class Solution {
public:
    vector<int> closestRoom(vector<vector<int>>& rooms, vector<vector<int>>& queries) {
        auto cmp = [](const vector<int>& a, const vector<int>& b){
            return a[1] < b[1];
        };
        auto cmp2 = [](const vector<int>& a, const int& target){
            return a[1] < target;
        };
        sort(rooms.begin(), rooms.end(), cmp);
        int n = queries.size();
        vector<int> sortedQueries(n);
        for(int i = 0;i<n;i++) {
            sortedQueries[i] = i;
        }
        auto cmp3 = [&](const int& a, const int& b){
            return queries[a][1] < queries[b][1];
        };
        sort(sortedQueries.begin(), sortedQueries.end(), cmp3);
        vector<int> ret(n);
        auto it = rooms.begin();
        for(auto idx: sortedQueries) {
            auto q = queries[idx];
            auto target = q[1];
            auto prefered = q[0];
            it = lower_bound(it, rooms.end(), target, cmp2);
            if(it == rooms.end()) {
                ret[idx] = -1;
            } else {
                auto room = (*it)[0];
                auto k = abs(prefered - room);
                for(auto it2 = it+1;it2 != rooms.end();it2++) {
                    auto nextRoom = (*it2)[0];
                    auto next = abs(prefered - nextRoom);
                    if(next < k || (next == k && nextRoom < room)) {
                        k = next;
                        room = nextRoom;
                    }
                }
                ret[idx] = room;
            }
        }
        return ret;
    }
};