package main

import (
	"fmt"
	"math/rand"
	"time"
)

func main() {
	rand.Seed(time.Now().Unix())
	if n := rand.Intn(10); n == 0 {
		println("少し小さすぎます", n)
	} else if n > 5 {
		println("大きすぎます", n)
	} else {
		println("いい感じの数字です", n)
	}
	fmt.Println(n) // compile error
}
