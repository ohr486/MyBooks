package main

import (
	"flag"
	"github.com/go-chi/chi"
	"net/http"
	"ohr486.net/math_server/calc"
	"strconv"
)

func main() {
	var port string
	flag.StringVar(&port, "port", "8080", "The port math_server listens on. Defaults to 8080.")
	flag.Parse()

	r := chi.NewRouter()
	r.Get("/", func(w http.ResponseWriter, r *http.Request) {
		expression := r.URL.Query().Get("expression")
		result, err := calc.Process(expression)
		if err != nil {
			w.WriteHeader(http.StatusBadRequest)
			w.Write([]byte(err.Error()))
			return
		}
		w.Write([]byte(strconv.FormatFloat(result, 'g', -1, 64)))
	})
	http.ListenAndServe(":"+port, r)
}
