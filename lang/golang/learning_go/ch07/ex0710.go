package main

import "fmt"

type Employee struct {
	Name string
	ID   string
}

func (e Employee) Description() string {
	return fmt.Sprintf("%s (%s)", e.Name, e.ID)
}

type Manager struct {
	Employee
	Reports []Employee
}

func (m Manager) FindNewEmployees() []Employee {
	newEmployees := []Employee{
		Employee{
			"石田三成",
			"13112",
		},
		Employee{
			"徳川家康",
			"13115",
		},
	}
	return newEmployees
}

func main() {
	m := Manager{
		Employee: Employee{
			Name: "上杉謙信",
			ID:   "12345",
		},
		Reports: []Employee{},
	}
	var eOK Employee = m.Employee
	fmt.Println(eOK)
	// var eFail Employee = m // compile error
}
