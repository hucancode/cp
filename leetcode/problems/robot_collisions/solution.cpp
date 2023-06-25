class Solution {
public:
    vector<int> survivedRobotsHealths(vector<int>& positions, vector<int>& healths, string directions) {
        int n = positions.size();
        vector<int> ret;
        ret.reserve(n);
        vector<int> task(n);
        iota(task.begin(), task.end(), 0);
        sort(task.begin(), task.end(), [&](int a,int b){
           return positions[a] < positions[b]; 
        });
        vector<int> right;
        right.reserve(n);
        for(auto i: task) {
            if(directions[i] == 'R') {
                right.push_back(i);
            } else {
                while(!right.empty() && healths[i]) {
                    int j = right.back();
                    right.pop_back();
                    //cout<<"robot["<<i<<"] "<<healths[i]<<" vs ""robot["<<j<<"] "<<healths[j]<<endl;
                    if(healths[j] == healths[i]) {
                        healths[j] = 0;
                        healths[i] = 0;
                    } else if(healths[j] > healths[i]) {
                        healths[j]--;
                        healths[i] = 0;
                        right.push_back(j);
                    } else {
                        healths[j] = 0;
                        healths[i]--;
                    }
                }
            }
        }
        for(auto h: healths) {
            if(h) {
                ret.push_back(h);
            }
        }
        return ret;
    }
};