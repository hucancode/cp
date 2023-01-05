class Solution {
public:
    int maxSum(vector<int>& nums1, vector<int>& nums2) {
        int n = nums1.size();
        int m = nums2.size();
        const int INF = 1e9+7;
        int u = 0;
        int v = 0;
        long long su = nums1[u];
        long long sv = nums2[v];
        while(u < n-1 || v < m - 1) {
            if(nums1[u] == nums2[v]) {
                su = sv = max(su,sv);
            }
            if(u == n-1) {
                v++;
                sv += nums2[v];
            } else if(v == m-1) {
                u++;
                su += nums1[u];
            } else if(nums1[u+1] < nums2[v+1]) {
                u++;
                su += nums1[u];
            } else {
                v++;
                sv += nums2[v];
            }
        }
        return max(su,sv)%INF;
    }
};