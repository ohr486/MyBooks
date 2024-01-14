package main

import "fmt"

func main() {
	totalWins := map[string]int{}
	totalWins["ライターズ"] = 1
	totalWins["ナイツ"] = 2
	fmt.Println(totalWins["ライターズ"])
	fmt.Println(totalWins["ミュージシャンズ"])
	totalWins["ミュージシャンズ"]++
	fmt.Println(totalWins["ミュージシャンズ"])
	totalWins["ナイツ"] = 3
	fmt.Println(totalWins["ナイツ"])
}
