class Solution {
    fun letterCombinations(digits: String): List<String> {
        if(digits.isEmpty())
            return emptyList()
        val dict = mapOf('2' to "abc", '3' to "def",
            '4' to "ghi", '5' to "jkl", '6' to "mno",
            '7' to "pqrs", '8' to "tuv", '9' to "wxyz")
        return digits.map { d -> dict[d]!! }
            .fold(listOf("")) { acc, next -> 
                acc.map { x -> next.map { y -> x + y } }
                    .flatten()
            }
    }
}