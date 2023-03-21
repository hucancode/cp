class Solution {
public:
    bool isFlush(vector<char>& suits) {
        for(int i = 1;i<5;i++) {
            if(suits[i] != suits[0]) return false;
        }
        return true;
    }
    int countDup(vector<int>& ranks) {
        int ret = 0;
        sort(ranks.begin(), ranks.end());
        int dup = 1;
        for(int i = 1;i<5;i++) {
            if(ranks[i] == ranks[i-1]) {
                dup++;
            } else {
                dup = 1;
            }
            ret = max(dup, ret);
        }
        return ret;
    }
    string bestHand(vector<int>& ranks, vector<char>& suits) {
        if(isFlush(suits)) {
            return "Flush";
        }
        auto x = countDup(ranks);
        if(x >= 3) {
            return "Three of a Kind";
        }
        if(x >= 2) {
            return "Pair";
        }
        return "High Card";
    }
};