package main

import "fmt"

func main() {
	words := []string{"hi", "salutations", "hello"}
	for _, word := range words {
		switch wordLen := len(word); {
		case wordLen < 5:
			fmt.Println(word, "は短い単語です")
		case wordLen > 10:
			fmt.Println(word, "は長すぎる単語です")
		default:
			fmt.Println(word, "はちょうどよい長さの単語です")
		}
	}
}
