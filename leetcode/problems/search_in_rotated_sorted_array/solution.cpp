class Solution {
public:
    int search(vector<int>& a, int target) {
        int l = 0;
        int r = a.size() - 1;
        int n = a.size();
        if(n == 1 || a[0] < a[n-1]) {
            cout<<"not rotated"<<endl;
            auto it = lower_bound(a.begin(), a.end(), target);
            if(it == a.end()) {
                return -1;
            }
            if(*it != target) {
                return -1;
            }
            return distance(a.begin(), it);
        }
        while(l<r) {
            int m = (l+r)/2;
            if(a[m] > a[l]) {
                l = m;
            } else if(a[m] < a[l]) {
                r = m;
            } else if(a[m] == a[l]) {
                break;
            }
        }
        cout<<"rotated, min at a["<<r<<"] = "<<a[r]<<", max at a["<<l<<"] = "<<a[l]<<endl;
        if(a[r] < a[l]) {
            if(target >= a[0]) {
                cout<<"search at left half a[0]~"<<*(a.begin()+r-1)<<endl;
                auto it = lower_bound(a.begin(), a.begin() + r, target);
                if(it == a.begin() + r) {
                    return -1;
                }
                if(*it != target) {
                    return -1;
                }
                return distance(a.begin(), it);
            } else {
                cout<<"search at right half "<<*(a.begin()+r)<<"~"<<endl;
                auto it = lower_bound(a.begin()+r, a.end(), target);
                if(it == a.end()) {
                    return -1;
                }
                if(*it != target) {
                    return -1;
                }
                return distance(a.begin(), it);
            }
        }
        return -1;
    }
};