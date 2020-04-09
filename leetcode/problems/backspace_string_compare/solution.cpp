class Solution {
public:
    bool backspaceCompare(string s, string t) {
        int i, j, n, m;
        n = s.size();
        m = t.size();
        int b = 0;
        for(i = n-1;i>=0;i--)
        {
            if(s[i] == '#')
            {
                b++;
            }
            else if(b > 0)
            {
                s[i] = '_';
                b--;
            }
        }
        b = 0;
        for(j = m-1;j>=0;j--)
        {
            if(t[j] == '#')
            {
                b++;
            }
            else if(b > 0)
            {
                t[j] = '_';
                b--;
            }
        }
        i = j = 0;
        while(i < n || j < m)
        {
            char cs = i<n?s[i]:'/';
            char ct = j<m?t[j]:'/';
            if(cs == '#' || cs == '_')
            {
                i++;
                continue;
            }
            if(ct == '#' || ct == '_')
            {
                j++;
                continue;
            }
            if(s[i] == t[j])
            {
                i++;
                j++;
            }
            else
            {
                return false;
            }
        }
        return true;
    }
};