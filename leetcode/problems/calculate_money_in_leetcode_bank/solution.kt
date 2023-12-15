class Solution {
    fun totalMoney(n: Int): Int {
        val progressSum = { first: Int, d: Int, n: Int -> 
            (first*2 + d*(n-1))*n/2
        }
        val weekCount = n/7;
        val lastWeekDayCount = n%7;
        return progressSum(progressSum(1,1,7),7,weekCount) + 
            progressSum(weekCount+1,1,lastWeekDayCount)
    }
}