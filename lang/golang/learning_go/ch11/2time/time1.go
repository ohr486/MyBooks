package main

import (
	"fmt"
	"time"
)

func main() {
	d := 2*time.Hour + 30*time.Minute + 45*time.Second
	fmt.Println(d)
}
