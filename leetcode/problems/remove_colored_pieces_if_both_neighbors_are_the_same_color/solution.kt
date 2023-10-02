class Solution {
    fun winnerOfGame(colors: String): Boolean {
        var a = 0
        for(w in colors.windowed(3)) 
            if(w.all {it == 'A'}) a++ else if(w.all {it == 'B'}) a--
        return a > 0
    }
}