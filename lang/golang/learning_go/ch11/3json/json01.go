package main

import (
	"encoding/json"
	"fmt"
)

func main() {
	f := struct {
		Name string
		Age  int
	}{}

	err := json.Unmarshal([]byte(`{"name": "小野小町", "occupation": "歌人", "age": 20}`), &f)
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Printf("%+v", f)
}
