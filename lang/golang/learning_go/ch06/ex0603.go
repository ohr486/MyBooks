package main

import "fmt"

func main() {
	type person struct {
		firstName  string
		MiddleName *string
		LastName   string
	}

	s := "Perry"
	p := person{
		firstName:  "Pat",
		MiddleName: &s,
		LastName:   "Peterson",
	}
	fmt.Println(p)
	fmt.Println(*p.MiddleName)
}
