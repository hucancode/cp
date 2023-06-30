#define UNUSED(mask, pos) ((mask & (1<<pos)) == 0)
class Solution {
public:
    bool isSolvable(vector<string>& words, string result) {
        map<char, int> weights;
        set<char> leads;
        auto addWeight = [&](string str, int k) {
            if(str.size() > 1) {
                leads.insert(str[0]);
            }
            reverse(str.begin(), str.end());
            for(auto c: str) {
                weights[c] += k;
                k *= 10;
            }
        };
        for(auto w: words) {
            addWeight(w, 1);
        }
        addWeight(result, -1);
        vector<pair<int,char>> vars;
        for(auto kv: weights) {
            if(kv.second == 0) continue;
            vars.emplace_back(kv.second, kv.first);
        }
        int n = vars.size();
        if(n == 0) {
            return true;
        }
        sort(vars.begin(), vars.end());
        stack<tuple<int, int, bool>> q;
        int sum = 0;
        int mask = 0;
        auto noHope = [&](int sum, int mask, int i) {
            if(sum > 0 || i >= n-1) {
                return true;
            }
            for(int j = n-1;j>i;j--) {
                for(int d = 9;d>=0;d--) {
                    if(UNUSED(mask, d)) {
                        sum += d*vars[j].first;
                        mask |= 1<<d;
                    }
                }
            }
            return sum < 0;
        };
        for(int j = 0;j<10;j++) {
            q.emplace(0, j, true);
        }
        while(!q.empty()) {
            int i, digit, w;
            char c;
            bool forward;
            tie(i, digit, forward) = q.top();
            q.pop();
            tie(w, c) = vars[i];
            int maski = 1<<digit;
            if(!forward) {
                mask &= ~maski;
                sum -= w*digit;
                continue;
            }
            if(digit == 0 && leads.find(c) != leads.end()) {
                continue;
            }
            mask |= maski;
            sum += w*digit;
            if(i == n-1 && sum == 0) {
                return true;
            }
            q.emplace(i, digit, false);
            if(noHope(sum, mask, i)) {
                continue;
            }
            for(int d = 0;d<10;d++) {
                if(UNUSED(mask, d)) {
                    q.emplace(i+1, d, true);
                }
            }
        }
        return false;
    }
};