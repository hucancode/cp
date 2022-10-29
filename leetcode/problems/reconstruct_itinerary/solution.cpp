typedef pair<int, int> pi;
typedef vector<string> vs;
typedef vector<vs> vvs;
class Solution {
public:
    bool canTravel(string& from, string& to, map<string, vector<string>>& edges) {
        stack<string> st;
        st.push(from);
        map<string, bool> visited;
        while(!st.empty()) {
            auto top = st.top();
            st.pop();
            if(visited[top]) {
                continue;
            }
            visited[top] = true;
            if(top == to) {
                return true;
            }
            for(const auto& edge: edges[top]) {
                st.push(edge);
            }
        }
        return false;
    }
    vector<string> findItinerary(vector<vector<string>>& tickets) {
        map<string, vector<string>> edges;
        for(auto& ticket: tickets) {
            edges[ticket[0]].emplace_back(move(ticket[1]));
        }
        for(auto& item: edges) {
            sort(item.second.rbegin(), item.second.rend());
        }
        vs ret;
        ret.emplace_back("JFK");
        while(true) {
            auto& u = *ret.rbegin();
            auto& neighbors = edges[u];
            if(neighbors.empty()) {
                break;
            }
            if(neighbors.size() == 1) {
                ret.emplace_back(neighbors[0]);
                neighbors.clear();
                continue;
            }
            if(canTravel(*neighbors.rbegin(), u, edges)) {
                ret.emplace_back(*neighbors.rbegin());
                neighbors.pop_back();
            } else {
                ret.emplace_back(*(neighbors.end()-2));
                neighbors.erase(neighbors.end()-2);
            }
        }
        return ret;
    }
};