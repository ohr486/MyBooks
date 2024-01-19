package main

import "fmt"

type MyInt int

func main() {
	var i any
	var mine MyInt = 20
	i = mine
	i3 := i.(string) // panic
	fmt.Println(i3)
}
