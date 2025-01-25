package piglatin

import (
	"testing"
)

type TestCase struct {
	description string
	input       string
	expected    string
}

var testCases = []TestCase{
	{
		"simple word beginning with a",
		"apple",
		"appleay",
	},
	{
		"simple word beginning with e",
		"ear",
		"earay",
	},
	{
		"simple word beginning with i",
		"igloo",
		"iglooay",
	},
	{
		"simple word beginning with o",
		"object",
		"objectay",
	},
	{
		"simple word beginning with u",
		"under",
		"underay",
	},
	{
		"word beginning with a single consonant",
		"pig",
		"igpay",
	},
	{
		"word beginning with multiple consonants",
		"latin",
		"atinlay",
	},
	{
		"word beginning with consonant cluster",
		"stupid",
		"upidstay",
	},
	{
		"word beginning with qu",
		"quick",
		"ickquay",
	},
	{
		"word beginning with qu and a preceding consonant",
		"squeal",
		"ealsquay",
	},
	{
		"word beginning with th",
		"therapy",
		"erapythay",
	},
	{
		"word beginning with thr",
		"thrush",
		"ushthray",
	},
	{
		"word beginning with sch",
		"school",
		"oolschay",
	},
	{
		"word beginning with yt",
		"yttria",
		"yttriaay",
	},
	{
		"word beginning with xr",
		"xray",
		"xrayay",
	},
	{
		"y is treated like a consonant at the beginning of a word",
		"yellow",
		"ellowyay",
	},
	{
		"y is treated like a vowel at the end of a consonant cluster",
		"rhythm",
		"ythmrhay",
	},
	{
		"y as second letter in two letter word",
		"my",
		"ymay",
	},
	{
		"phrase",
		"quick fast run",
		"ickquay astfay unray",
	},
	{
		"all vowels",
		"aeiou",
		"aeiouay",
	},
	{
		"all consonants",
		"thrsty",
		"ythrstay",
	},
	{
		"all consonants before y",
		"myth",
		"ythmay",
	},
	{
		"all consonants before y and a",
		"myth a",
		"ythmay aay",
	},
	{
		"all consonants before y and a and e",
		"myth a e",
		"ythmay aay eay",
	},
	{
		"all consonants before y and a and e and i",
		"myth a e i",
		"ythmay aay eay iay",
	},
	{
		"all consonants before y and a and e and i and o",
		"myth a e i o",
		"ythmay aay eay iay oay",
	},
}

func TestPigLatin(t *testing.T) {
	for _, tc := range testCases {
		t.Run(tc.description, func(t *testing.T) {
			if actual := Sentence(tc.input); actual != tc.expected {
				t.Fatalf("Sentence(%q) = %q, want %q", tc.input, actual, tc.expected)
			}
		})
	}
}
func BenchmarkSentence(b *testing.B) {
	if testing.Short() {
		b.Skip("skipping benchmark in short mode.")
	}
	for i := 0; i < b.N; i++ {
		for _, tc := range testCases {
			Sentence(tc.input)
		}
	}
}
