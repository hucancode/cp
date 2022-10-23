class Solution {
public:
    int hourToInt(string hour) {
        int h = 0;
        int m = 0;
        h += (hour[0] - '0')*10 + (hour[1] - '0');
        m += (hour[3] - '0')*10 + (hour[4] - '0');
        return h*60+m;
    }
    bool haveConflict(vector<string>& event1, vector<string>& event2) {
        int a = hourToInt(event1[0]);
        int b = hourToInt(event1[1]);
        int c = hourToInt(event2[0]);
        int d = hourToInt(event2[1]);
        if(a <= c) {
            return b >= c;
        }
        return d >= a;
    }
};