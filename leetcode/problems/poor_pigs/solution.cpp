class Solution {
public:
    int poorPigs(int buckets, int minutesToDie, int minutesToTest) {
        // I didn't came up with the solution, I gave up and found a good explaination here
        // https://leetcode.com/problems/poor-pigs/discuss/94273/solution-with-detailed-explanation
        int test = minutesToTest/minutesToDie + 1;
        return ceil(log10(buckets)/log10(test));
    }
};