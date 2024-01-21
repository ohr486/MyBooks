package main

import (
	"errors"
	"fmt"
	"math/rand"
	"time"
)

func timeLimit() (int, error) {
	var result int
	var err error
	done := make(chan struct{})
	go func() {
		result, err = doSomeWork()
		close(done)
	}()

	select {
	case <-done:
		return result, err
	case <-time.After(2 * time.Second):
		return 0, errors.New("タイムアウトしました")
	}
}

func doSomeWork() (int, error) {
	rand.Seed(time.Now().UnixNano())
	n := rand.Intn(4)
	fmt.Println("n:", n)
	time.Sleep(time.Duration(n) * time.Second)
	result := 33
	return result, nil
}

func main() {
	result, err := timeLimit()

	if err != nil {
		fmt.Println(err)
	} else {
		fmt.Printf("結果: %d\n", result)
	}
}
