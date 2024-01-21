package main

import (
	"fmt"
	"math/rand"
	"strings"
	"time"
)

func main() {
	rand.Seed(time.Now().UnixNano())
	funcs := prepareFunctions()
	s := "Time files like an arrow."
	fmt.Println("オリジナル:", s)
	r := convertData(s, funcs)
	fmt.Printf("一番早く到着した結果: %v\n\n", r)

	fmt.Println("他のゴルーチンが終了するのを待つためにスリープ")
	time.Sleep(1 * time.Second)
	fmt.Println("main終了")
}

type message struct {
	s        string
	fromFunc int
}

func convertData(s string, converters []func(string) message) message {
	done := make(chan struct{})
	resultChan := make(chan message)
	for _, f := range converters {
		go func(f func(string) message) {
			r := f(s)
			select {
			case resultChan <- r:
				fmt.Printf("結果が戻ってきたのでresultChanに入れた後: %v\n", r)
			case <-done:
				fmt.Println("case <-done選択", r.fromFunc)
			}
			fmt.Println("無名関数終了", r)
		}(f)
	}

	r := <-resultChan
	close(done)
	return r
}

func randomPeriod() time.Duration {
	t := time.Millisecond * time.Duration(rand.Intn(4)+2)
	return t
}

func prepareFunctions() []func(string) message {
	funcNo1 := func(a string) message {
		sleepRandomPeriod(1)
		b := strings.ToLower(a)
		fmt.Println("処理完了(1):", b)
		return message{b, 1}
	}
	funcNo2 := func(a string) message {
		sleepRandomPeriod(2)
		b := strings.ToUpper(a)
		fmt.Println("処理完了(2):", b)
		return message{b, 2}
	}
	funcNo3 := func(a string) message {
		sleepRandomPeriod(3)
		b := strings.ReplaceAll(a, "i", "I")
		fmt.Println("処理完了(3):", b)
		return message{b, 3}
	}
	funcNo4 := func(a string) message {
		sleepRandomPeriod(4)
		b := strings.ReplaceAll(a, "e", "E")
		a = strings.ReplaceAll(a, "r", "L")
		fmt.Println("処理完了(4):", b)
		return message{b, 4}
	}

	return []func(string) message{funcNo1, funcNo2, funcNo3, funcNo4}
}

func sleepRandomPeriod(funcNum int) {
	t := randomPeriod()
	fmt.Println(funcNum, "will sleep: ", t)
	time.Sleep(t)
}
