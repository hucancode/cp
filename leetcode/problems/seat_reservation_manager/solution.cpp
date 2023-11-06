class SeatManager {
  vector<int> available;
  vector<bool> reserved;
public:
    SeatManager(int n) {
      available.push_back(1);
      reserved.resize(n+1, false);
    }
    
    int reserve() {
      int ret = available[0];
      available[0]++;
      if(ret+1 >= reserved.size() ||
        reserved[ret+1] || 
        (available.size() > 1 && available[1] == available[0])) {
        available.erase(available.begin());
      }
      reserved[ret] = true;
      //cout<<"reserve return "<<ret<<endl;
      return ret;
    }
    
    void unreserve(int seatNumber) {
      auto it = lower_bound(available.begin(), available.end(), seatNumber);
      available.insert(it, seatNumber);
      reserved[seatNumber] = false;
      //cout<<"unreserve "<<seatNumber<<endl;
    }
};

/**
 * Your SeatManager object will be instantiated and called as such:
 * SeatManager* obj = new SeatManager(n);
 * int param_1 = obj->reserve();
 * obj->unreserve(seatNumber);
 */