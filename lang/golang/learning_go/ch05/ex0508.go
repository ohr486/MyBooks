package main

import (
	"errors"
	"fmt"
)

func divAndRemainder(numerator int, denominator int) (result int, remainder int, err error) {
	if denominator == 0 {
		err = errors.New("0で割ることはできません")
		return
	}
	result, remainder = numerator/denominator, numerator%denominator
	return
}

func main() {
	rs, rm, _ := divAndRemainder(5, 2)
	fmt.Println(rs, rm)
}
