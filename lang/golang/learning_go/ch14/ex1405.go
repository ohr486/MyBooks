package main

import (
	"fmt"
	"reflect"
)

func main() {
	i := 10
	iv := reflect.ValueOf(&i)
	fmt.Println(iv)
	ivv := iv.Elem()
	fmt.Println(ivv)
	ivv.SetInt(20)
	fmt.Println(ivv)
}
