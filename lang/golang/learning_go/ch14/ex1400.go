package main

import (
	"fmt"
	"reflect"
)

func main() {
	fmt.Println("----- TypeOf -----")
	{
		v := 0
		w := 0.1

		vType := reflect.TypeOf(v)
		fmt.Println(vType)
		wType := reflect.TypeOf(w)
		fmt.Println(wType)
	}
}
