package main

import (
	"fmt"
	"ohr486.net/package_example/formatter"
	"ohr486.net/package_example/math"
)

func main() {
	num := math.Double(2)
	output := print.Format(num)
	fmt.Println(output)
}
