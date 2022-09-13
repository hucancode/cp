class Solution {
public:
    bool validTail(int k) {
        // cout<<"check tail "<<bitset<8>(k)<<endl;
        const int a = 0b10000000;
        const int b = 0b10111111;
        return (k&a) == a && (k|b) == b;
    }
    int calculateLen(int k) {
        const int a[4] = {
            0b00000000,
            0b11000000,
            0b11100000,
            0b11110000
        };
        const int b[4] = {
            0b01111111,
            0b11011111,
            0b11101111,
            0b11110111
        };
        for(int i = 0;i<4;i++) {
            if((k&a[i]) == a[i] && (k|b[i]) == b[i]) {
                return i;
            }
        }
        return 5;
    }
    bool validUtf8(vector<int>& data) {
        // cout<<"check ";
        // for(const auto& x: data) {
        //     cout<<bitset<8>(x)<<",";
        // }
        // cout<<endl;
        int n = data.size();
        int i = 0;
        while(i < n) {
            int len = calculateLen(data[i]);
            //cout<<"processing trunk with len "<<len<<endl;
            if(len > 3) {
                //cout<<"return false because invalid head"<<endl;
                return false;
            }
            if(n-i+1 <= len) {
                //cout<<"return false because not enough tail"<<endl;
                return false;
            }
            i++;
            while(len--) {
                if(!validTail(data[i++])) {
                    //cout<<"return false because invalid tail"<<endl;
                    return false;
                }
            }
        }
        return true;
    }
};