class Solution {
public:
    string orderlyQueue(string s, int k) {
        // important observation:
        // if k=1, only n rotation of s is possible
        // if k>=2, we take s[1] and rotate n-1 times, that will result in s[0] and s[1] swapped
        // if we want to swap s[i] and s[i+1], just rotate i times and repeat the strategy above
        // we can safely say, if k>=2, any permutation of s is possible
        // so if k>=2, the best result is s with all elements sorted 
        if(k>1) {
            sort(s.begin(), s.end());
            return s;
        }
        // n max = 1000, we can safely traverse all possible solution and return the best 
        string best = s;
        string current = s;
        for(int i = 0;i<s.size();i++) {
            char c = current[0];
            current.erase(current.begin());
            current.push_back(c);
            if(current < best) {
                best = current;
            }
        }
        return best;
    }
};