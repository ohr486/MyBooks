package main

import "fmt"

type Stack[T any] struct {
	vals []T
}

func (s *Stack[T]) Push(val T) {
	s.vals = append(s.vals, val)
}

func (s *Stack[T]) Pop() (T, bool) {
	if len(s.vals) == 0 {
		var zero T
		return zero, false
	}
	top := s.vals[len(s.vals)-1]
	s.vals = s.vals[:len(s.vals)-1]
	return top, true
}

func (s Stack[T]) Contains(val T) bool {
	for _, v := range s.vals {
		// compile error
		if v == val {
			return true
		}
	}
	return false
}

func main() {
	var intStack Stack[int]
	intStack.Push(10)
	intStack.Push(20)
	intStack.Push(30)
	fmt.Println(intStack.Contains(10))
	fmt.Println(intStack.Contains(5))
}
