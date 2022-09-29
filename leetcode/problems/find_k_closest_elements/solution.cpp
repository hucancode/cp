class Solution {
public:
    vector<int> findClosestElements(vector<int>& arr, int k, int x) {
        int n = arr.size();
        auto it = lower_bound(arr.begin(), arr.end(), x);
        if(it == arr.end()) {
            it--;
        }
        if(it != arr.begin() && abs(*it - x) >= abs(*(it-1) -x)) {
            it--;
        }
        auto left = it;
        auto right = it;
        while(distance(left, right) < k-1) {
            bool canGoLeft = left != arr.begin();
            bool canGoRight = right != arr.end() - 1;
            if(!canGoLeft && !canGoRight) {
                break;
            }
            if(!canGoLeft) {
                right++;
                continue;
            }
            if(!canGoRight) {
                left--;
                continue;
            }
            //cout<<"l = "<<*left<<" - r = "<<*right<<endl;
            auto dl = abs(*(left-1) - x);
            auto dr = abs(*(right+1) - x);
            //cout<<"go left, dl = "<<dl<<", go right, dr = "<<dr<<endl;
            if(dl > dr) {
                right++;
                //cout<<"go right"<<endl;
            } else {
                left--;
                //cout<<"go left"<<endl;
            }
        }
        vector<int> ret(left, right+1);
        return ret;
    }
};