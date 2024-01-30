package main

import (
	"fmt"
	"reflect"
)

func main() {
	{
		var i int
		changeInt(&i)
		fmt.Println("i:", i)
	}
	{
		var i int
		changeIntReflect(&i)
		fmt.Println("i:", i)
	}
}

func changeInt(i *int) {
	*i = 20
}

func changeIntReflect(i *int) {
	iv := reflect.ValueOf(i)
	iv.Elem().SetInt(20)
}
