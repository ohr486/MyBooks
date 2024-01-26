package main

import (
	"encoding/json"
	"fmt"
	"log"
	"os"
	"os/exec"
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

const (
	inFileName  = "testdata/data2.json"
	outFileName = "testdata/data2-out.json"
)

func main() {
	r, err := os.Open(inFileName)
	if err != nil {
		log.Fatal(err)
	}
	defer r.Close()

	var dec *json.Decoder
	dec = json.NewDecoder(r)

	outFile, err := os.Create(outFileName)
	if err != nil {
		log.Fatal(err)
	}

	encoder := json.NewEncoder(outFile)

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
	outFile.Close()

	result, err := exec.Command("cat", outFileName).Output()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("出力ファイルを表示します\n----------")
	fmt.Printf("%s", result)
	fmt.Println("----------")

	command := "diff -s " + inFileName + " " + outFileName + " | cat"
	result, err = exec.Command("sh", "-c", command).Output()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("diffの結果:\n%s", result)
}
