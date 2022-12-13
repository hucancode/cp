// Use Segment Tree to manage majority element in range [l,r]
// Majority element = element that appears at least half of the time in the range
// Theorem: in the range [l,r], if exist majority element x, then x also is majority element of at least 1 range [l,m],[m+1,r] (l<=m<=r)
// The theorem above guaranteed that:
// - If range [l,r] has majority element, the tree will surely return it
// - If range [l,r] do not have majority element, the tree will return a random element which not neccessarily has the highest occurence. But we are OK with it.

class SegmentTree {
    map<int, vector<int>> idx;
    vector<int> data;
    vector<int> tree;
public:
    SegmentTree(vector<int>& arr) {
        data = arr;
        buildOccurence();
        tree.resize(data.size()*4);
        build(1, 0, data.size() - 1);
    }

    void buildOccurence() {
        int n = data.size();
        for(int i = 0;i<n;i++) {
            auto x = data[i];
            idx[x].push_back(i);
        }
    }

    int countOccurence(int left, int right, int x) {
        if(x == 0) {
            return 0;
        }
        auto i = lower_bound(idx[x].begin(), idx[x].end(), left);
        auto j = upper_bound(i, idx[x].end(), right);
        return distance(i,j);
    }

    void build(int root, int left, int right) {
        if(left > right) {
            return;
        }
        if(left == right) {
            tree[root] = data[left];
            return;
        }
        auto len = right - left + 1;
        auto childl = root*2;
        auto childr = childl+1;
        auto mid = (left + right)/2;
        build(childl, left, mid);
        build(childr, mid + 1, right);
        auto a = tree[childl];
        auto b = tree[childr];
        auto ca = countOccurence(left, right, a);
        auto cb = countOccurence(left, right, b);
        if(ca > cb) {
            tree[root] = a;
        } else {
            tree[root] = b;
        }
    }

    pair<int, int> queryMajority(int left, int right) {
        auto x = query(left, right);
        auto count = countOccurence(left, right, x);
        //cout<<"query majority "<<left<<"-"<<right<<" return "<<x<<"(count "<<count<<")"<<endl;
        return make_pair(x, count);
    }

    int query(int left, int right) {
        return query(1, 0, data.size() - 1, left, right);
    }

    int query(int root, int left, int right, int qleft, int qright) {
        if(qleft > qright) {
            return 0;
        }
        if(left == qleft && right == qright) {
            //cout<<"query left "<<left<<" right "<<right<<" qleft "<<qleft<<" qright "<<qright<<endl;
            //cout<<"return "<<tree[root]<<endl;
            return tree[root];
        }
        auto len = right - left + 1;
        auto mid = (left+right)/2;
        auto childl = root*2;
        auto childr = childl+1;
        auto a = query(childl, left, mid, qleft, min(mid, qright));
        auto b = query(childr, mid+1, right, max(qleft, mid+1), qright);
        auto ca = countOccurence(qleft, qright, a);
        auto cb = countOccurence(qleft, qright, b);
        if(ca > cb) {
            //cout<<"query left "<<left<<" right "<<right<<" qleft "<<qleft<<" qright "<<qright<<endl;
            //cout<<"return "<<a<<endl;
            return a;
        }
        //cout<<"query left "<<left<<" right "<<right<<" qleft "<<qleft<<" qright "<<qright<<endl;
        //cout<<"return "<<b<<endl;
        return b;
    }
};

class MajorityChecker {
    SegmentTree* tree;
public:
    MajorityChecker(vector<int>& arr) {
        tree = new SegmentTree(arr);
    }

    int query(int left, int right, int threshold) {
        auto [x, count] = tree->queryMajority(left, right);
        if(count < threshold) {
            return -1;
        }
        return x;
    }
};

/**
 * Your MajorityChecker object will be instantiated and called as such:
 * MajorityChecker* obj = new MajorityChecker(arr);
 * int param_1 = obj->query(left,right,threshold);
 */