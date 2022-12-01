class Solution {
public:
    vector<vector<int>> reconstructQueue(vector<vector<int>>& people) {
        // observation: insert short people into array doesn't affect tall people's score
        // => traverse people from tall to short, as long as a person found a position, it's guaranteed that's correct position
        // people with same height, whoever has k smaller must stand in front of people with bigger k
        // => sort people from tall to short, from small k to big k
        vector<int> f;
        sort(people.begin(), people.end(), [](const vector<int>& a, const vector<int>& b) {
            if(a[0] > b[0]) {
                return true;
            }
            if(a[0] < b[0]) {
                return false;
            }
            return a[1] < b[1];
        });
        vector<vector<int>> ret;
        for(auto& person: people) {
            auto h = person[0];
            auto k = person[1];
            ret.insert(ret.begin()+k, person);
        }
        return ret;
    }
};