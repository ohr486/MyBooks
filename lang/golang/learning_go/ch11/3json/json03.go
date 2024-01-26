package main

import (
	"encoding/json"
	"fmt"
	"os"
)

func main() {
	type Person struct {
		Name string `json:"name"`
		Age  int    `json:"age"`
	}
	dateToFile := Person{
		Name: "フレッド",
		Age:  40,
	}

	tmpFile, err := os.CreateTemp(os.TempDir(), "sample-")
	if err != nil {
		panic(err)
	}
	defer os.Remove(tmpFile.Name())

	err = json.NewEncoder(tmpFile).Encode(dateToFile)
	if err != nil {
		panic(err)
	}
	err = tmpFile.Close()
	if err != nil {
		panic(err)
	}

	tmpFile2, err := os.Open(tmpFile.Name())
	if err != nil {
		panic(err)
	}
	var dataFromFile Person
	err = json.NewDecoder(tmpFile2).Decode(&dataFromFile)
	if err != nil {
		panic(err)
	}
	err = tmpFile2.Close()
	if err != nil {
		panic(err)
	}

	fmt.Printf("ファイルから読み込んだデータ: %+v\n", dataFromFile)
}
