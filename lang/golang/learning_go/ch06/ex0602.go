package main

import "fmt"

func main() {
	type person struct {
		firstName  string
		MiddleName *string
		LastName   string
	}

	p := person{
		firstName:  "Pat",
		MiddleName: "Perry", // compile error
		LastName:   "Peterson",
	}
	fmt.Println(p)
}
