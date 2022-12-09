class Solution {
public:
    typedef vector<int> vi;
    typedef vector<vi> vvi;
    typedef vector<vvi> v3i;
    typedef tuple<int,int,int> state;
    const int MOUSE_WIN = 1;
    const int CAT_WIN = 2;
    const int DRAW = 0;
    const int UNKNOWN = -1;
    const int MOUSE_MOVE = 0;
    const int CAT_MOVE = 1;
    v3i f;
    int search(vvi& graph, int mouse, int cat, int turn) {
        if(f[mouse][cat][turn] != UNKNOWN) {
            return f[mouse][cat][turn];
        }
        if(mouse == 0) {
            f[mouse][cat][turn] = MOUSE_WIN;
            return MOUSE_WIN;
        }
        if(cat == mouse) {
            f[mouse][cat][turn] = CAT_WIN;
            return CAT_WIN;
        }
        f[mouse][cat][turn] = DRAW;
        int best;
        bool canWin = false;
        bool canDraw = false;
        int nextTurn = (turn + 1)%2;
        if(turn == MOUSE_MOVE) {
            for(auto next: graph[mouse]) {
                int result = search(graph, next, cat, nextTurn);
                if(result == DRAW) {
                    canDraw = true;
                } else if(result == MOUSE_WIN) {
                    canWin = true;
                    break;
                }
            }
            if(canWin) {
                best = MOUSE_WIN;
            } else if(canDraw) {
                best = DRAW;
            } else {
                best = CAT_WIN;
            }
        } else {
            
            for(auto next: graph[cat]) {
                if(next == 0) {
                    continue;
                }
                int result = search(graph, mouse, next, nextTurn);
                if(result == DRAW) {
                    canDraw = true;
                } else if(result == CAT_WIN) {
                    canWin = true;
                    break;
                }
            }
            if(canWin) {
                best = CAT_WIN;
            } else if(canDraw) {
                best = DRAW;
            } else {
                best = MOUSE_WIN;
            }
        }
        f[mouse][cat][turn] = best;
        return best;
    }
    int countDraw() {
        int n = f.size();
        int ret = 0;
        for(auto m = 0;m<n;m++) {
            for(auto c = 0;c<n;c++) {
                for(auto t = 0;t<2;t++) {
                    if(f[m][c][t] == DRAW) {
                        ret++;
                    }
                }
            }
        }
        return ret;
    }
    int clearDraw() {
        int n = f.size();
        int ret = 0;
        for(auto m = 0;m<n;m++) {
            for(auto c = 0;c<n;c++) {
                for(auto t = 0;t<2;t++) {
                    if(f[m][c][t] == DRAW) {
                        f[m][c][t] = UNKNOWN;
                    }
                }
            }
        }
        return ret;
    }
    int catMouseGame(vector<vector<int>>& graph) {
        int n = graph.size();
        f.resize(n, vvi(n, vi(2, UNKNOWN)));
        int lastDrawCount = -1;
        while(true) {
            search(graph, 1, 2, MOUSE_MOVE);
            // this nasty loop code is neccessary, there are flaw in my search function
            // some DRAW are actually a MOUSE_WIN or CAT_WIN, so we need to double check until our calculation stable
            int drawCount = countDraw();
            if(drawCount == lastDrawCount) {
                break;
            }
            lastDrawCount = drawCount;
            clearDraw();
        }
        auto ret = search(graph, 1, 2, MOUSE_MOVE);
        return ret;
        for(auto m = 0;m<n;m++) {
            for(auto c = 0;c<n;c++) {
                auto result = f[m][c][MOUSE_MOVE];
                if(result != DRAW && result != UNKNOWN) {
                    cout<<"mouse at "<<m<<" cat at "<<c<<" MOUSE_MOVE, result = "<<f[m][c][MOUSE_MOVE]<<endl;
                }
                result = f[m][c][CAT_MOVE];
                if(result != DRAW && result != UNKNOWN) {
                    cout<<"mouse at "<<m<<" cat at "<<c<<" CAT_MOVE, result = "<<f[m][c][CAT_MOVE]<<endl;
                }
            }
        }
        return ret;
    }
};