package main

import "fmt"

func main() {
	x := []int{1, 2, 3, 4}
	y := make([]int, 2)
	copy(y, x[1:])
	fmt.Println(y)
}
