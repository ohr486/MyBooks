package main

import "fmt"

type BuildInOrdered interface {
	string | int | int8 | int16 | int32 | int64 | float32 | float64 |
		uint | uint8 | uint16 | uint32 | uint64 | uintptr
}

func Min[T BuildInOrdered](v1, v2 T) T {
	if v1 < v2 {
		return v1
	}
	return v2
}

func main() {
	a := 10
	b := 20
	fmt.Println(Min(a, b))
}
