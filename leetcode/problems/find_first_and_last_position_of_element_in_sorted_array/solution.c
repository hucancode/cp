

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int* searchRange(int* nums, int numsSize, int target, int* returnSize)
{
    *returnSize = 2;
    int* ret = malloc(2*sizeof(int));
    ret[0] = -1;
    ret[1] = -1;
    if(numsSize < 1)
    {
        return ret;
    }
    int l, r, m;
    l = 0;
    r = numsSize - 1;
    m = (l+r)/2;
    while(l<=r)
    {
        m = (l+r)/2;
        if(nums[m] > target)
        {
            r = m-1;
        }
        else if(nums[m] < target)
        {
            l = m+1;
        }
        else
        {
            break;
        }
    }
    if(nums[m] != target)
    {
        return ret;
    }
    l = 0;
    r = numsSize - 1;
    int m0 = m;
    int m1, m2;
    m1 = m0;
    while(l <= m)
    {
        int mid = (l + m)/2;
        if(nums[mid] < target)
        {
            l = mid+1;
        }
        else if(nums[mid] == target)
        {
            m1 = mid;
            m = mid-1;
        }
    }
    m = m0;
    m2 = m0;
    while(m <= r)
    {
        int mid = (m + r)/2;
        if(nums[mid] > target)
        {
            r = mid-1;
        }
        else if(nums[mid] == target)
        {
            m2 = mid;
            m = mid+1;
        }
    }
    ret[0] = m1;
    ret[1] = m2;
    return ret;
}

