package main

import (
	"fmt"
	"reflect"
)

func main() {
	type Foo struct {
		A int
		B string
	}

	var x int
	xt := reflect.TypeOf(x)
	fmt.Println(xt.Name())
	fmt.Printf("%T\n", xt.Name())

	f := Foo{}
	ft := reflect.TypeOf(f)
	fmt.Println(ft.Name())
	fmt.Printf("%T\n", ft.Name())

	xpt := reflect.TypeOf(&x)
	fmt.Println(xpt.Name())
	fmt.Printf("%T\n", xpt.Name())

	fmt.Println("-----")
	fmt.Println(xt.Kind())
	fmt.Println(ft.Kind())
	fmt.Println(xpt.Kind())

	fmt.Println("-----")
	fmt.Printf("%T(%d)\n", xt.Kind(), xt.Kind())
	fmt.Printf("%T(%d)\n", ft.Kind(), ft.Kind())
	fmt.Printf("%T(%d)\n", xpt.Kind(), xpt.Kind())
}
