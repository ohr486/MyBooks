package main

import (
	"fmt"
	"time"
)

type Counter struct {
	total       int
	lastUpdated time.Time
}

func (c *Counter) Increment() {
	c.total++
	c.lastUpdated = time.Now()
}

func (c Counter) String() string {
	return fmt.Sprintf("合計: %d, 更新: %v", c.total, c.lastUpdated)
}

func doUpdateWrong(c Counter) {
	c.Increment()
	fmt.Println("NG:", c.String())
}

func doUpdateRight(c *Counter) {
	c.Increment()
	fmt.Println("OK:", c.String())
}

func main() {
	var c Counter
	doUpdateWrong(c)
	fmt.Println("main:", c.String())
	doUpdateRight(&c)
	fmt.Println("main:", c.String())
}
