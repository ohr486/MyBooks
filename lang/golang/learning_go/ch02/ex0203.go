package main

import "fmt"

const x int64 = 10

const (
	idkey   = "id"
	nameKey = "name"
)

const z = 20 * 10

func main() {
	const y = "hello"

	fmt.Println(x)
	fmt.Println(y)

	x = x + 1 // compile error
	y = "bye" // compile error

	fmt.Println(x)
	fmt.Println(y)
}
