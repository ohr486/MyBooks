package main

func main() {
	type person struct {
		firstName  string
		MiddleName string
		LastName   string
	}

	p := person{
		firstName:  "Pat",
		MiddleName: "Perry",
		LastName:   "Peterson",
	}
}
