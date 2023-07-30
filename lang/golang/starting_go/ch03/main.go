package main

import (
	"fmt"
	"math"
)

var pn = 100

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
	pn = pn + 1
	fmt.Printf("n=%d\n", pn)

	// 3.7
	n = 256
	var b byte = byte(n)
	fmt.Printf("b=%d\n", b)

	n = -1
	b = byte(n)
	fmt.Printf("b=%d\n", b)

	fmt.Printf("uint32 max value = %d\n", math.MaxUint32)

	fmt.Printf("value = %v\n", 1.0000000000000000)
	fmt.Printf("value = %v\n", 1.0000000000000001)
	fmt.Printf("value = %v\n", 1.0000000000000002)
	fmt.Printf("value = %v\n", 1.0000000000000003)
	fmt.Printf("value = %v\n", 1.0000000000000004)
	fmt.Printf("value = %v\n", 1.0000000000000005)
	fmt.Printf("value = %v\n", 1.0000000000000006)
	fmt.Printf("value = %v\n", 1.0000000000000007)
	fmt.Printf("value = %v\n", 1.0000000000000008)
	fmt.Printf("value = %v\n", 1.0000000000000009)

	fmt.Printf("value = %v\n", float32(1.0000000000000000))
	fmt.Printf("value = %v\n", float32(1.0000000000000001))
	fmt.Printf("value = %v\n", float32(1.0000000000000002))
	fmt.Printf("value = %v\n", float32(1.0000000000000003))
	fmt.Printf("value = %v\n", float32(1.0000000000000004))
	fmt.Printf("value = %v\n", float32(1.0000000000000005))
	fmt.Printf("value = %v\n", float32(1.0000000000000006))
	fmt.Printf("value = %v\n", float32(1.0000000000000007))
	fmt.Printf("value = %v\n", float32(1.0000000000000008))
	fmt.Printf("value = %v\n", float32(1.0000000000000009))

	r := '松'
	fmt.Printf("%v", r)

	s := "Goの文字列"
	fmt.Printf("%v", s)

	s = `
Goの
RAW文字列リテラルによる
複数行に渡る
文字列
`
	fmt.Printf("%v", s)
}
