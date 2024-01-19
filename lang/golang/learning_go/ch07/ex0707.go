package main

import "fmt"

func main() {

	type MailCategory int

	const (
		Uncategorized MailCategory = iota
		Personal
		Spam
		Social
		Advertisements
	)

	m := Personal
	fmt.Println(m)
	m = Uncategorized
	fmt.Println(m)

	type BitField int

	const (
		Field1 BitField = 1 << iota
		Field2
		Field3
		Field4
	)

	fmt.Println("Field1:", Field1)
	fmt.Println("Field2:", Field2)
	fmt.Println("Field3:", Field3)
	fmt.Println("Field4:", Field4)
}
