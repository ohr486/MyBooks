package main

import "fmt"

func main() {
	a := 10
	goto skip // compile error
	b := 20
skip:
	c := 30
	fmt.Println(a, b, c)
	if c > a {
		goto inner // compile error
	}
	if a < b {
	inner:
		fmt.Println("aはbより小さい")
	}
}
