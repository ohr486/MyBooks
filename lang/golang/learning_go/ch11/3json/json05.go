package main

import (
	"bytes"
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
	r, err := os.Open("testdata/data2.json")
	if err != nil {
		log.Fatal(err)
	}

	var dec *json.Decoder
	dec = json.NewDecoder(r)

	var b bytes.Buffer
	encoder := json.NewEncoder(&b)

	for dec.More() {
		var o Order
		err := dec.Decode(&o)
		if err != nil {
			log.Fatal(err)
		}
		err = encoder.Encode(o)
		if err != nil {
			log.Fatal(err)
		}
	}
	out := b.String()
	fmt.Printf("out:\n%v\n", out)
}
