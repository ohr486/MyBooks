package main

import (
	"fmt"
)

func main() {
	// 3.5
	n := 1 + 2 + 3
	fmt.Println(n)
	fmt.Println("Hello, World!")
	fmt.Println("Hello, Golang!")
	fmt.Println("My", "name", "is", "Taro")

	fmt.Printf("数値=%d\n", 5)
	fmt.Printf("%d年%d月%d日\n", 2023, 7)
	fmt.Printf("%d年%d月%d日\n", 2023, 7, 30, 17)
	fmt.Printf("数値=%v 文字列=%v 配列=%v\n", 5, "Golang", [...]int{1, 2, 3})
	fmt.Printf("数値=%#v 文字列=%#v 配列=%#v\n", 5, "Golang", [...]int{1, 2, 3})
	fmt.Printf("数値=%T 文字列=%T 配列=%T\n", 5, "Golang", [...]int{1, 2, 3})

	print("Hello, World!")
	println("Hello, World!")
	println(1, 2, 3)

	// 3.6

}
