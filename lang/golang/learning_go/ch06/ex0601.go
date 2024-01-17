package main

import "fmt"

func main() {
	x := 10
	pointerToX := &x
	fmt.Println(pointerToX)
	fmt.Println(*pointerToX)
	z := 5 + *pointerToX
	fmt.Println(z)

	var y *int
	fmt.Println(y == nil)
	// fmt.Println(*y) // panic

	var a = new(int)
	fmt.Println(a == nil)
	fmt.Println(*a)
}
