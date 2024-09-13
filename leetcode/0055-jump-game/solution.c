int max(int a, int b) {
    return a>b?a:b;
}
bool canJump(int* a, int n) {
    int max_distance = 0;
    for(int i = 0;i<n;i++) {
        if(i > max_distance) {
            return false;
        }
        int new_oppotunity = i+a[i];
        max_distance = max(max_distance, new_oppotunity);
    }
    return true;
}
