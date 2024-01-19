package main

type MyInt int

func main() {
	var i any
	var mine MyInt = 20
	i = mine
	i4 := i.(int) // panic
	println(i4)
}
