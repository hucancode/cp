void moveZeroes(int* a, int n)
{
    int i = 0;
    int j = 0;
    while(i<n)
    {
        if(a[i] != 0)
        {
            a[j++] = a[i];
        }
        i++;
    }
    while(j<n)
    {
        a[j++] = 0;
    }
}

