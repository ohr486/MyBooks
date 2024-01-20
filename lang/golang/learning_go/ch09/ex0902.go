package main

import "fmt"

type T1 struct {
	x int
	S string
}

func (f T1) Hello() string {
	return "hello"
}

func (f T1) goodbye() string {
	return "goodbye"
}

type T2 = T1

func MakeT2() T2 {
	t2 := T2{
		x: 20,
		S: "Hello",
	}
	var f T2 = t2
	fmt.Println(f.Hello())
	return t2
}

func main() {
	a := MakeT2()
	fmt.Println(a)
}
