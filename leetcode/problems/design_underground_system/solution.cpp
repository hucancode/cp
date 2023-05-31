class UndergroundSystem {
    map<int, pair<string, int>> cardIn;
    map<pair<string, string>, pair<double, int>> travelLog;
public:
    UndergroundSystem() {
    }
    
    void checkIn(int id, string stationName, int t) {
        cardIn[id] = {stationName, t};
    }
    
    void checkOut(int id, string stationName, int t) {
        auto& data = travelLog[{cardIn[id].first, stationName}];
        data.first += t - cardIn[id].second;
        data.second += 1;
    }
    
    double getAverageTime(string startStation, string endStation) {
        auto& data = travelLog[{startStation, endStation}];
        return data.first/data.second;
    }
};

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * UndergroundSystem* obj = new UndergroundSystem();
 * obj->checkIn(id,stationName,t);
 * obj->checkOut(id,stationName,t);
 * double param_3 = obj->getAverageTime(startStation,endStation);
 */