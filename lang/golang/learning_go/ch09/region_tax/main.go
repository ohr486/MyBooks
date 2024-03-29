package main

import (
	"fmt"
	"github.com/learning-go-book/simpletax"
	"github.com/shopspring/decimal"
	"os"
)

func main() {
	amount, _ := decimal.NewFromString(os.Args[1])
	zip := os.Args[2]
	percent, err := simpletax.TaxForZip(zip)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	total := amount.Add(amount.Mul(percent)).Round(2)
	fmt.Println(total)
}
