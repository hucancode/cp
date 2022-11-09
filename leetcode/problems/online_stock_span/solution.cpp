class StockSpanner {
public:
    stack<pair<int,int>> span;
    int idx;
    StockSpanner() {
        span.emplace(1e5+1, -1);
        idx = -1;
    }
    
    int next(int price) {
        int l = idx;
        idx++;
        while(!span.empty() && span.top().first <= price) {
            l = min(l, span.top().second);
            //cout<<"pop "<<span.top().first<<"-"<<span.top().second<<endl;
            span.pop();
        }
        span.emplace(price, l);
        //cout<<"push "<<price<<"-"<<l<<endl;
        return idx-l;
    }
};

/**
 * Your StockSpanner object will be instantiated and called as such:
 * StockSpanner* obj = new StockSpanner();
 * int param_1 = obj->next(price);
 */