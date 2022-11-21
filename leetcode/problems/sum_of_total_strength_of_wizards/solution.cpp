class Solution {
public:
    int totalStrength(vector<int>& strength) {
        typedef long long ll;
        const int INF = 1e9+7;
        int n = strength.size();
        long long ret = 0;
        vector<ll> ps_l(n + 1), pm_l(n + 1); // prefix sum and prefix mul from the left.
        vector<ll> ps_r(n + 1), pm_r(n + 1); // ... from the right.
        vector<int> st; // mono-increasing stack.
        for (ll i = 0; i < n; ++i) {
            ps_l[i + 1] = (ps_l[i] + strength[i]) % INF;
            pm_l[i + 1] = (pm_l[i] + (i + 1) * strength[i]) % INF;
        }
        for (ll i = n - 1; i >= 0; --i) {
            ps_r[i] = (ps_r[i + 1] + strength[i]) % INF;
            pm_r[i] = (pm_r[i + 1] + (n - i) * strength[i]) % INF;
        }        
        for (int right = 0; right <= n; ++right) {
            while(!st.empty() && (right == n || strength[st.back()] >= strength[right])) {
                int pivot = st.back();
                st.pop_back();
                ll left = st.empty() ? 0 : st.back() + 1;
                ll left_sum = pm_l[pivot + 1] - pm_l[left] - left * (ps_l[pivot + 1] - ps_l[left]) % INF;
                left_sum = (left_sum + INF)%INF;
                ll right_sum = pm_r[pivot + 1] - pm_r[right] - (n - right) * (ps_r[pivot + 1] - ps_r[right]);
                right_sum = (right_sum + INF)%INF;
                ll all_sum = (right_sum * (pivot - left + 1) + left_sum * (right - pivot)) % INF;
                ret += all_sum * strength[pivot];
                ret %= INF;
            }
            st.push_back(right);
        }
        return ret;
    }
};