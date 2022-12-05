class FenwickTree {
    vector<int> data;
    public:
    FenwickTree(int n) {
        data.resize(n, 0);
    }
    int goLeft(int i) {
        return (i & (i+1)) -1;
    }
    int goUp(int i) {
        return i | (i+1);
    }
    void add(int i) {
        while(i < data.size()) {
            data[i] += 1;
            i = goUp(i);
        }
    }
    int query(int i) {
        i = min(i, (int)data.size() - 1);
        int count = 0;
        while(i >= 0) {
            count += data[i];
            i = goLeft(i);
        }
        return count;
    }
};
class LUPrefix {
    FenwickTree* tree;
    int best;
public:
    LUPrefix(int n) {
        best = 0;
        tree = new FenwickTree(n+1);
    }
    
    void upload(int video) {
        tree->add(video);
    }
    
    int longest() {
        while(true) {
            int next = best+1;
            if(tree->query(next) != next) {
                break;
            }
            best = next;    
        }
        return best;
    }
};

/**
 * Your LUPrefix object will be instantiated and called as such:
 * LUPrefix* obj = new LUPrefix(n);
 * obj->upload(video);
 * int param_2 = obj->longest();
 */