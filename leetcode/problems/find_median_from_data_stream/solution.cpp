class MedianFinder {
public:
    vector<int> a;
    bool sorted;
    MedianFinder() {
        a.reserve(50000);
        sorted = false;
    }
    
    void addNum(int num) {
        if(sorted) {
            a.insert(upper_bound(a.begin(), a.end(), num), num);
        } else {
            a.push_back(num);
        }
    }
    
    double findMedian() {
        if(!sorted) {
            sort(a.begin(), a.end());
            sorted = true;
        }
        int n = a.size();
        if(n%2 == 1) {
            return a[n/2];
        }
        return (a[n/2] + a[n/2 - 1])/2.0;
    }
};

/**
 * Your MedianFinder object will be instantiated and called as such:
 * MedianFinder* obj = new MedianFinder();
 * obj->addNum(num);
 * double param_2 = obj->findMedian();
 */