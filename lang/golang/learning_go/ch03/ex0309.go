package main

import "fmt"

func main() {
	x := [...]int{5, 6, 7, 8}
	y := x[:2]
	z := x[2:]
	d := x[:]
	x[0] = 10
	fmt.Println("x:", x)
	fmt.Println("y:", y)
	fmt.Println("z:", z)
	fmt.Println("d:", d)
}
