package main

import "fmt"

func main() {
	var x = [...]int{1, 2, 3}
	var y = [3]int{1, 2, 3}
	fmt.Println(x == y)

	x[0] = 10
	fmt.Println(x[0])

	fmt.Println(len(x))

}
