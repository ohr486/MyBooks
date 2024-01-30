package main

import (
	"fmt"
	"reflect"
)

func main() {
	s := []string{"a", "b", "c"}
	sv := reflect.ValueOf(s)
	s2 := sv.Interface().([]string)

	fmt.Println(s)
	fmt.Println(sv)
	fmt.Printf("%T\n", sv)
	fmt.Println(s2)
	fmt.Printf("%T\n", s2)

	s3 := sv.Interface()
	fmt.Printf("s3: %v\n", s3)
	fmt.Printf("s3の型: %T\n", s3)
}
