package piglatin

import "strings"

func isVowel(c uint8) bool {
	return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}
func indexOfVowel(word string) int {
	for i, c := range word {
		if isVowel(uint8(c)) {
			return i
		}
	}
	return -1
}

func Sentence(sentence string) string {
	ret := ""
	for _, word := range strings.Split(sentence, " ") {
		beginWithXrYt := len(word) >= 2 && (word[:2] == "xr" || word[:2] == "yt")
		indexOfQu := strings.Index(word, "qu")
		indexOfY := strings.Index(word, "y")
		vowelIndex := indexOfVowel(word)
		noVowelBeforeQu := indexOfQu != -1 && indexOfVowel(word[:indexOfQu]) == -1
		noVowelBeforeY := indexOfY > 0 && indexOfVowel(word[:indexOfY]) == -1
		n := 0
		if noVowelBeforeQu {
			n = indexOfQu + 2
		} else if noVowelBeforeY {
			n = indexOfY
		} else if !beginWithXrYt && vowelIndex != -1 {
			n = vowelIndex
		}
		ret += word[n:] + word[:n] + "ay "
	}
	return strings.TrimSpace(ret)
}
