class Allocator {
    int poolSize;
    vector<tuple<int,int,int>> blocks;
public:
    Allocator(int n) {
        poolSize = n;
        blocks.emplace_back(0,0,0);
    }

    void print() {
        int start,end,id;
        cout<<"blocks: ";
        for(auto item: blocks) {
            tie(start,end,id) = item;
            cout<<"["<<start<<"~"<<end<<"/"<<id<<"] ";
        }
        cout<<endl;
    }
    
    int allocate(int size, int mID) {
        //cout<<"alloc "<<size<<" id: "<<mID<<endl;
        int ret = -1;
        int n = blocks.size();
        int a, b;
        int start,end,id;
        for(int i = 0;i<n;i++) {
            tie(start,end,id) = blocks[i];
            a = end;
            if(i != n-1) {
                tie(start,end,id) = blocks[i+1];
                b = start;
            } else {
                b = poolSize;
            }
            if(b-a >= size) {
                ret = a;
                blocks.insert(blocks.begin()+i+1, make_tuple(a, a+size, mID));
                break;
            }
        }
        //print();
        return ret;
    }
    int free(int mID) {
        //cout<<"free "<<mID<<endl;
        int ret = 0;
        for(auto it = blocks.begin();it!= blocks.end();) {
            int start, end, id;
            tie(start, end, id) = *it;
            if(id == mID) {
                it = blocks.erase(it);
                ret += end - start;
            } else {
                it++;
            }
        }
        //print();
        return ret;
    }
};

/**
 * Your Allocator object will be instantiated and called as such:
 * Allocator* obj = new Allocator(n);
 * int param_1 = obj->allocate(size,mID);
 * int param_2 = obj->free(mID);
 */