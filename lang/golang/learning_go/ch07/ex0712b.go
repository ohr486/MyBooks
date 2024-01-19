package main

import "fmt"

func main() {
	var i any
	i = 20
	fmt.Println(i)
	i = "Hello"
	fmt.Println(i)
	i = struct {
		FirstName string
		LastName  string
	}{"信玄", "武田"}
	fmt.Println(i)
}
