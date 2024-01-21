package main

import (
	"fmt"
	"time"
)

func countTo(max int) <-chan int {
	ch := make(chan int)
	go func() {
		for i := 0; i < max; i++ {
			ch <- i
		}
		close(ch)
	}()
	return ch
}

func main() {
	for i := range countTo(10) {
		fmt.Print(i, " ")
		if i > 5 {
			break
		}
	}
	fmt.Println()
	doSomethingTalkingLongTime()
}

func doSomethingTalkingLongTime() {
	time.Sleep(5 * time.Second)
}
