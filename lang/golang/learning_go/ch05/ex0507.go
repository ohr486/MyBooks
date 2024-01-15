package main

import (
	"errors"
	"fmt"
)

func divAndRemainder(numerator int, denominator int) (result int, remainder int, err error) {
	result, remainder = 20, 30
	if denominator == 0 {
		return numerator, denominator, errors.New("0で割ることはできません")
	}
	return numerator / denominator, numerator % denominator, nil
}

func main() {
	rs, rm, _ := divAndRemainder(5, 2)
	fmt.Println(rs, rm)
}
