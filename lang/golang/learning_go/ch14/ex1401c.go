package main

import (
	"fmt"
	"reflect"
)

func main() {
	f := func(a, b int) int {
		return a + b
	}
	fmt.Println(f(3, 9))

	ft := reflect.TypeOf(f)
	fmt.Println(ft)
	fmt.Println(ft.NumIn())

	x := 3
	xt := reflect.TypeOf(x)
	fmt.Println(xt)
	fmt.Printf("%T\n", xt)
	fmt.Println(xt.NumIn())
}
