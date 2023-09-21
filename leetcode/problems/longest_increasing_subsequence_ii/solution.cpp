class SegmentTree {
private:
    vector<int> data;
    int n;
public:
    SegmentTree() {
    }
    void build(int len) {
        n = len;
        data.resize(4*n, 0);
    }
    void build(vector<int>& input) {
        n = input.size();
        data.resize(4*n);
        build(input, 1, 0, n-1);
    }
    int query(int l, int r) {
        if(l < 0 || l >= n || r < 0 || r >= n || l > r) return 0;
        return query(1, 0, n-1, l, r);
    }
    void update(int pos, int x) {
        if(pos < 0 || pos >= n) return;
        return update(1, 0, n-1, pos, x);
    }
protected:
    virtual int neutralElement() = 0;
    virtual int combine(int a, int b) = 0;
    void build(vector<int>& input, int root, int l, int r) {
        if (l == r) {
            data[root] = input[l];
            return;
        }
        int m = (l+r)/2;
        int childl = root*2;
        int childr = childl+1;
        build(input, childl, l, m);
        build(input, childr, m+1, r);
        data[root] = combine(data[childl], data[childr]);
    }
    int query(int root, int tl, int tr, int ql, int qr) {
        if (tr < ql || tl > qr) {
            return neutralElement();
        }
        if(ql <= tl  && tr <= qr) {
            return data[root];
        }
        int m = (tl+tr)/2;
        int childl = root*2;
        int childr = childl+1;
        auto a = query(childl, tl, m, ql, qr);
        auto b = query(childr, m+1, tr, ql, qr);
        return combine(a,b);
    }
    void update(int root, int tl, int tr, int pos, int x) {
        if (tl == tr) {
            data[root] = x;
            return;
        }
        int m = (tl+tr)/2;
        int childl = root*2;
        int childr = childl+1;
        if (pos <= m) {
            update(childl, tl, m, pos, x);
        } else {
            update(childr, m+1, tr, pos, x);
        }
        data[root] = combine(data[childl], data[childr]);
    }
};
// Segment Tree to answer sum query
class SegmentTreeSum: public SegmentTree {
protected:
    virtual int neutralElement() override {
        return 0;
    }
    virtual int combine(int a, int b) override {
        return a+b;
    }
};
// Segment Tree to answer max query
class SegmentTreeMax: public SegmentTree {
protected:
    virtual int neutralElement() override {
        return INT_MIN;
    }
    virtual int combine(int a, int b) override {
        return max(a,b);
    }
};
// Segment Tree to answer min query
class SegmentTreeMin: public SegmentTree {
protected:
    virtual int neutralElement() override {
        return INT_MAX;
    }
    virtual int combine(int a, int b) override {
        return min(a,b);
    }
};
class Solution {
public:
    int lengthOfLIS(vector<int>& nums, int k) {
        int ret = 0;
        SegmentTreeMax tree;
        tree.build(100001);
        for(auto x: nums) {
            int l = max(0, min(100000, x-k));
            int r = max(0, min(100000, x-1));
            int best = tree.query(l, r)+1;
            //cout<<"query "<<l<<"-"<<r<<" return "<<best<<endl;
            tree.update(x, best);
            ret = max(ret, best);
        }
        return ret;
    }
};