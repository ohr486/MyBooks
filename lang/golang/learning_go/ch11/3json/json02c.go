package main

import (
	"encoding/json"
	"fmt"
	"log"
	"os"
	"time"
)

type Order struct {
	ID          string    `json:"id"`
	DateOrdered time.Time `json:"date_ordered"`
	CustomerID  string    `json:"customer_id"`
	Items       []Item    `json:"items"`
}

type Item struct {
	ID   string `json:"id"`
	Name string `json:"name"`
}

func main() {
	file, err := os.Open("testdata/data.json")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var o Order
	d := json.NewDecoder(file)
	d.Decode(&o)
	fmt.Printf("%+v\n", o)
}
