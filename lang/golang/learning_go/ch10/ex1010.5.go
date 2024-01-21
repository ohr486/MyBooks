package main

import (
	"fmt"
	"math/rand"
	"strings"
	"time"
)

type strConv func(string) string

func main() {
	rand.Seed(time.Now().UnixNano())

	strConvFuncs := []strConv{strings.ToLower, strings.ToUpper}
	s := "Time files like an arrow."
	fmt.Println("オリジナル:", s)

	r := []string{s}
	r2 := getVariations(s, strConvFuncs)
	r = append(r, r2...)
	fmt.Println("結果:", r)
}

func getVariations(s string, converters []strConv) []string {
	done := make(chan struct{})
	chs := []chan string{make(chan string), make(chan string)}

	for i, f := range converters {
		go func(s string, f strConv, ch chan string, i int) {
			s = f(s)
			fmt.Printf("変換結果 %d: %s\n", i, s)
			ch <- s
			close(ch)
		}(s, f, chs[i], i)
	}

	r := []string{}
	for {
		select {
		case v, ok := <-chs[0]:
			if !ok {
				chs[0] = nil
				fmt.Println("continueする: chan0")
				continue
			}
			fmt.Println("変換結果をappend:", v)
			r = append(r, v)
		case v, ok := <-chs[1]:
			if !ok {
				chs[1] = nil
				fmt.Println("continueする: chan1")
				continue
			}
			fmt.Println("appending:", v)
			r = append(r, v)
		case <-done:
			fmt.Println("case <-done選択")
			return r
		}

		if len(r) >= len(chs) {
			close(done)
		}
	}
}

func randomPeriod() time.Duration {
	t := time.Millisecond * time.Duration(rand.Intn(4)+2)
	return t
}

func sleepRandomPeriod(funcNum int) {
	t := randomPeriod()
	fmt.Println(funcNum, "will sleep: ", t)
	time.Sleep(t)
}
