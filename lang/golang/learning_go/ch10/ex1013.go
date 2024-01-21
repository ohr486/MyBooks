package main

import (
	"fmt"
	"sync"
)

func processAndGather(processor func(int) int, data []int) []int {
	num := len(data)
	chResult := make(chan int, num)
	var wg sync.WaitGroup
	wg.Add(num)

	for _, v := range data {
		go func(v int) {
			defer wg.Done()
			chResult <- processor(v)
		}(v)
	}

	wg.Wait()
	close(chResult)

	var result []int
	for v := range chResult {
		result = append(result, v)
	}
	return result
}

func main() {
	inpValues := []int{1, 2, 3, 4, 5}
	outValues := processAndGather(
		func(j int) int { return j * j },
		inpValues)

	fmt.Println(outValues)
}
