package main

import "fmt"

func stringp(s string) *string {
	return &s
}

func main() {
	type person struct {
		firstName  string
		MiddleName *string
		LastName   string
	}

	p := person{
		firstName:  "Pat",
		MiddleName: stringp("Perry"),
		LastName:   "Peterson",
	}
	fmt.Println(p)
	fmt.Println(*p.MiddleName)
}
