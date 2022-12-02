class Solution {
public:
    string minInteger(string num, int k) {
        string ret;
        typedef pair<int,int> pi;
        priority_queue<pi, vector<pi>, greater<pi>> pq;
        int n = num.size();
        if(k>=n*(n+1)/2){ 
			sort(num.begin(), num.end());
			return num;
		}
        for(int i = 0;i<n;i++) {
            int m = min(i+k+1, n);
            char best = num[i];
            int bestj = i;
            for(int j = i+1;j<m;j++) {
                if(num[j] < best) {
                    best = num[j];
                    bestj = j;
                }
            }
            int cost = bestj - i;
            k -= cost;
            while(bestj > i) {
                swap(num[bestj], num[bestj-1]);
                bestj--;
            }
        }
        return num;
    }
};