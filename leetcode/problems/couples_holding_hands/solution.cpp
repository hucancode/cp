class Solution {
public:
    bool trySwap2(vector<int>& row, int i, int j) {
        int a = row[i-1]/2;
        int b = row[i]/2;
        int c = row[j-1]/2;
        int d = row[j]/2;
        if((a == c && b == d) || (a == d && b == c)) {
            swap(row[i], row[j]);
            return true;
        }
        return false;
    }
    bool trySwap1(vector<int>& row, int i, int j) {
        int a = row[i-1]/2;
        int b = row[i]/2;
        int c = row[j-1]/2;
        int d = row[j]/2;
        if(a == c) {
            swap(row[i], row[j-1]);
            return true;
        }
        if(a == d) {
            swap(row[i], row[j]);
            return true;
        }
        if(b == c) {
            swap(row[i-1], row[j-1]);
            return true;
        }
        if(b == d) {
            swap(row[i-1], row[j]);
            return true;
        }
        return false;
    }
    int minSwapsCouples(vector<int>& row) {
        int n = row.size();
        int ret = 0;
        for(int i =1;i<n-2;i+=2) {
            int ca1 = row[i-1]/2;
            int ca2 = row[i]/2;
            if(ca1 == ca2) {
                continue;
            }
            bool found = false;
            int cb1;
            int cb2;
            for(int j=i+2;j<n;j+=2) {
                if(trySwap2(row,i,j)) {
                    ret++;
                    found = true;
                    break;
                }
            }
            if(found) {
                continue;
            }
            for(int j=i+2;j<n;j+=2) {
                if(trySwap1(row,i,j)) {
                    ret++;
                    found = true;
                    break;
                }
            }
            if(found) {
                continue;
            }
            cout<<"something went wrong"<<endl;
        }
        return ret;
    }
};