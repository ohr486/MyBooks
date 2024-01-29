package main

import (
	"fmt"
	"reflect"
)

type Foo struct {
	A int
	B string
}

func main() {
	f := Foo{}
	ft := reflect.TypeOf(f)
	fmt.Println(ft)
	fmt.Println(ft.Name())
	fmt.Println(ft.Kind())
}
