class Solution {
    fun defangIPaddr(address: String): String {
        return address.split('.')
            .joinToString("[.]")
    }
}
