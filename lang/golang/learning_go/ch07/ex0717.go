package main

import (
	"fmt"
	"os"
)

type MyInt int

func main() {
	var i any
	var mine MyInt = 20
	i = mine
	i4, ok := i.(int)
	if !ok {
		err := fmt.Errorf("iの型(値:%v)が想定外です", i)
		fmt.Println(err.Error())
		os.Exit(1)
	}
	fmt.Println(i4)
}
