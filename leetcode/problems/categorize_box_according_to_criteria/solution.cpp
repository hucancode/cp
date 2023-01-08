class Solution {
public:
    string categorizeBox(int length, int width, int height, int mass) {
        long long volume = (long long)length*width*height;
        bool bulky = length >= 1e4 || width >= 1e4 || height >= 1e4 || volume >= 1e9;
        bool heavy = mass >= 100;
        if(bulky && heavy) {
            return "Both";
        }
        if(bulky) {
            return "Bulky";
        }
        if(heavy) {
            return "Heavy";
        }
        return "Neither";
    }
};