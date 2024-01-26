package main

import (
	"encoding/json"
	"fmt"
	"io"
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
	data := readData()
	var o Order
	err := json.Unmarshal(data, &o)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("%+v\n", o)
}

func readData() []byte {
	file, err := os.Open("testdata/data.json")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	bytes, err := io.ReadAll(file)
	if err != nil {
		log.Fatal(err)
	}
	return bytes
}
