package main

import "fmt"

func doBusinessLogic(val int) int {
	return val * val
}

func runThingsConcurrently(chIn <-chan int, chOut chan<- string) {
	for val := range chIn {
		go func(val int) {
			result := doBusinessLogic(val)
			resultString := fmt.Sprintf("%d -> %d", val, result)
			chOut <- resultString
		}(val)
	}
}

func main() {
	chIn := make(chan int)
	chOut := make(chan string)
	go runThingsConcurrently(chIn, chOut)

	vals := []int{1, 2, 3, 4, 5}
	for v := range vals {
		chIn <- v
	}

	i := 0
	for v := range chOut {
		fmt.Println(v)
		i++
		if len(vals) <= i {
			break
		}
	}

	close(chOut)
	close(chIn)
}
