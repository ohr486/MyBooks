package main

import (
	"fmt"
	"ohr486.net/circular_dependency_example/person"
)

func main() {
	bob := person.Person{PetName: "Flufy"}
	fmt.Println(bob.Pet())
}
