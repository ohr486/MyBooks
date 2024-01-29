package main

import (
	"fmt"
	"reflect"
)

func main() {
	var x int
	xpt := reflect.TypeOf(&x)
	fmt.Println(xpt.Name())
	fmt.Println(xpt.Kind())
	fmt.Println(xpt.Elem().Name())
	fmt.Println(xpt.Elem().Kind())

	s := xpt.Elem().Name()
	fmt.Println(reflect.TypeOf(s))
	c := xpt.Elem().Kind()
	fmt.Println(reflect.TypeOf(c))
}
