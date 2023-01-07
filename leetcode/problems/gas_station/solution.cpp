class Solution {
public:
    int canCompleteCircuit(vector<int>& gas, vector<int>& cost) {
        int n = gas.size();
        vector<int> benefit(n);
        for(int i = 0;i<n;i++) {
            benefit[i] = gas[i] - cost[i];
        }
        int i = 0;// travel from 0
        int j = 1%n;// end at 1
        int delta = benefit[0]; // how many gas left?
        while(i < n) {
            //cout<<"travel from "<<i<<" to "<<j<<", have "<<delta<<" gas left"<<endl;
            bool finished = j == i && delta >= 0;
            if(finished) {
                return i;
            }
            bool shouldStop = delta < 0 || j == i;
            if(shouldStop) {
                delta -= benefit[i];
                i++;
                if(i!=j) {
                    continue;
                }
            }
            delta += benefit[j];
            j = (j+1)%n;
        }
        return -1;
    }
};