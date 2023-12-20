#define UNUSED(mask, pos) ((mask & (1<<pos)) == 0)
class Solution {
public:
    bool isSolvable(vector<string>& words, string result) {
        vector<int> weights(26);
        set<int> leads;
        auto add_weight = [&](string str, int k) {
            if(str.size() > 1) {
                leads.insert(str[0]-'A');
            }
            reverse(str.begin(), str.end());
            for(auto c: str) {
                weights[c-'A'] += k;
                k *= 10;
            }
        };
        for(auto w: words) {
            add_weight(w, 1);
        }
        add_weight(result, -1);
        vector<pair<int,int>> vars;
        for(int i = 0;i<26;i++) {
            if(weights[i] == 0) continue;
            vars.emplace_back(weights[i], i);
        }
        int n = vars.size();
        if(n == 0) {
            return true;
        }
        sort(vars.begin(), vars.end());
        stack<tuple<int, int, bool>> q;
        int sum = 0;
        int mask = 0;
        auto no_hope = [&](int sum, int mask, int i) {
            if(sum > 0 || i >= n-1) {
                return true;
            }
            for(int j = n-1;j>i;j--) {
                for(int d = 9;d>=0;d--) {
                    sum += UNUSED(mask, d)*d*vars[j].first;
                    mask |= 1<<d;
                }
            }
            return sum < 0;
        };
        for(int j = 0;j<10;j++) {
            q.emplace(0, j, true);
        }
        while(!q.empty()) {
            int i, digit, w, c;
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
            if(digit == 0 && leads.contains(c)) {
                continue;
            }
            mask |= maski;
            sum += w*digit;
            if(i == n-1 && sum == 0) {
                return true;
            }
            q.emplace(i, digit, false);
            if(no_hope(sum, mask, i)) {
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