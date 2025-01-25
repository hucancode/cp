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
	words := strings.Split(sentence, " ")
	for _, word := range words {
		beginWithVowel := len(word) >= 1 && isVowel(word[0])
		beginWithXrYt := len(word) >= 2 && (word[:2] == "xr" || word[:2] == "yt")
		indexOfQu := strings.Index(word, "qu")
		indexOfY := strings.Index(word, "y")
		vowelIndex := indexOfVowel(word)
		noVowelBeforeQu := indexOfQu != -1 && indexOfVowel(word[:indexOfQu]) == -1
		noVowelBeforeY := indexOfY > 0 && indexOfVowel(word[:indexOfY]) == -1
		if noVowelBeforeQu {
			n := indexOfQu + 2
			ret += word[n:] + word[:n] + "ay"
		} else if noVowelBeforeY {
			n := indexOfY
			ret += word[n:] + word[:n] + "ay"
		} else if beginWithVowel || beginWithXrYt {
			ret += word + "ay"
		} else if vowelIndex != -1 {
			n := vowelIndex
			ret += word[n:] + word[:n] + "ay"
		} else {
			ret += word
		}
		ret += " "
	}
	return strings.TrimSpace(ret)
}
