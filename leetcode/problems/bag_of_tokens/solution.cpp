class Solution {
public:
    int bagOfTokensScore(vector<int>& tokens, int power) {
        sort(tokens.begin(), tokens.end());
        int n = tokens.size();
        int i = 0;
        int j = n - 1;
        int score = 0;
        while(i <= j) {
            if(power >= tokens[i]) {
                power -= tokens[i];
                score++;
                i++;
                continue;
            }
            if(tokens[j] > tokens[i] && score > 0) {
                score--;
                power += tokens[j];
                j--;
                continue;
            }
            break;
        }
        return score;
    }
};