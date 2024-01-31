package main

import "fmt"

type Integer interface {
	~int | ~int8 | ~int16 | ~int32 | ~int64 | ~uint | ~uint8 | ~uint16 | ~uint32 | ~uint64
}

func plusOneHundred[T Integer](in T) T {
	return in + 100
}

func main() {
	var a int = 0
	b := plusOneHundred(a)

	fmt.Printf("%T: %v\n", b, b)
}
