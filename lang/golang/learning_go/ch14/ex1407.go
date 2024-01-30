package main

import (
	"fmt"
	"reflect"
)

func hasNoValue(i interface{}) bool {
	iv := reflect.ValueOf(i)
	if !iv.IsValid() {
		return true
	}
	switch iv.Kind() {
	case reflect.Ptr, reflect.Slice, reflect.Map, reflect.Func, reflect.Interface:
		return iv.IsNil()
	default:
		return false
	}
}

func main() {
	var a interface{}
	fmt.Println(a == nil, hasNoValue(a))

	var b *int
	fmt.Println(b == nil, hasNoValue(b))

	var c interface{} = b
	fmt.Println(c == nil, hasNoValue(c))

	var d int
	fmt.Println(hasNoValue(d))

	var e interface{} = d
	fmt.Println(e == nil, hasNoValue(e))
}
