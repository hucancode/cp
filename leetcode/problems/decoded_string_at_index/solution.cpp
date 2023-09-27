class Solution {
public:
    string decodeAtIndex(string s, int k) {
        k--;
        stack<tuple<int,int,long long>> q;
        int n = s.size();
        int l = 0;
        int r = 0;
        long long size = 0;
        for(int i = 0;i<=n;i++) {
            if(i == n || s[i] > '0' && s[i] <= '9') {
                int rep = 1;
                if(i < n) {
                    rep = s[i] - '0';
                }
                size += r-l;
                q.emplace(l,r,size);
                size *= rep;
                r++;
                l = r;
            } else {
                r++;
            }
        }
        while(!q.empty()) {
            int l,r;
            long long size;
            tie(l,r,size) = q.top();
            q.pop();
            k %= size;
            //cout<<"check segment "<<l<<"-"<<r<<" size "<<size<<" search for index "<<k<<endl;
            int tail_size = r-l;
            long long head_size = size-tail_size;
            if(k < head_size) {
                continue;
            }
            k -= head_size;
            if(tail_size <= 0) {
                continue;
            }
            k += l;
            break;
        }
        return s.substr(k, 1);
    }
};