module main

import strings

fn is_vowel(c rune) bool {
    return match c {
      `a`, `e`, `i`, `o`, `u` { true }
        else { false }
    }
}

fn index_of_first_vowel(s string) ?int {
    for i := 0; i < s.len; i++ {
        if is_vowel(s[i]) {
            return i
        }
    }
    return none
}

fn translate(phrase string) string {
  return phrase.split(' ')
    .map(fn (word string) string {
      first_vowel_idx := index_of_first_vowel(word) or { -1 }
      begin_with_xr_yt := word[..2] == 'xr' || word[..2] == 'yt'
      qu_idx := word.index('qu') or { -1 }
      y_idx := word.index('y') or { -1 }
      no_vowel_before_qu := qu_idx >= 0 && (first_vowel_idx < 0 || qu_idx < first_vowel_idx)
      no_vowel_before_y := y_idx > 0 && (first_vowel_idx < 0 || y_idx < first_vowel_idx)
      mut n := 0
      if no_vowel_before_qu {
        n = qu_idx + 2
      } else if no_vowel_before_y {
        n = y_idx
      } else if !begin_with_xr_yt && first_vowel_idx >= 0 {
        n = first_vowel_idx
      }
      return word[n..] + word[..n] + 'ay'
    })
    .join(' ')
}
