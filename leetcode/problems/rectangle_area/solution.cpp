class Solution {
public:
    int computeArea(int x1, int y1, int x2, int y2) {
        int w = x2 - x1;
        int h = y2 - y1;
        return w*h;
    }
    int computeArea(int ax1, int ay1, int ax2, int ay2, int bx1, int by1, int bx2, int by2) {
        int overlapx = max(0, min(ax2, bx2) - max(ax1, bx1));
        int overlapy = max(0, min(ay2, by2) - max(ay1, by1));
        return computeArea(ax1, ay1, ax2, ay2) + computeArea(bx1, by1, bx2, by2) - overlapx*overlapy;
    }
};