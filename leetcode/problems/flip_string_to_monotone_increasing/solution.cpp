class Solution {
public:
    int minFlipsMonoIncr(string s) {
        int n = s.size();
        int f = 0; // f = min flip to make s monotone and the last item = 0
        int g = 0; // g = min flip to make s monotone and last item = 1
        for(auto c: s) {
            auto cost0 = c != '0';
            auto cost1 = c != '1';
            g = min(f, g) + cost1;
            f = f + cost0;
        }
        return min(f,g);
    }
};