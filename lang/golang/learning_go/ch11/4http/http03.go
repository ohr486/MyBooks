package main

import (
	"fmt"
	"log"
	"net/http"
)

func main() {
	personMux := http.NewServeMux()
	personMux.HandleFunc("/greet",
		func(w http.ResponseWriter, r *http.Request) {
			w.Write([]byte("こんにちは!\n"))
		})

	dogMux := http.NewServeMux()
	dogMux.HandleFunc("/greet",
		func(w http.ResponseWriter, r *http.Request) {
			w.Write([]byte("カワイイ子犬ちゃんだね!\n"))
		})

	mux := http.NewServeMux()
	mux.Handle("/person/", http.StripPrefix("/person", personMux))
	mux.Handle("/dog/", http.StripPrefix("/dog", dogMux))
	fmt.Printf("%s", "次でテスト:\ncurl http://localhost:8080/dog/greet\ncurl http://localhost:8080/person/greet\n")

	log.Fatal(http.ListenAndServe(":8080", mux))
}
