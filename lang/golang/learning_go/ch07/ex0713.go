package main

import "fmt"

type LinkedList struct {
	Value any
	Next  *LinkedList
}

func (ll *LinkedList) Insert(pos int, val any) *LinkedList {
	if ll == nil || pos == 0 {
		return &LinkedList{
			Value: val,
			Next:  ll,
		}
	}
	ll.Next = ll.Next.Insert(pos-1, val)
	return ll
}

func (ll *LinkedList) String() string {
	if ll == nil {
		return "<nil>"
	}
	return fmt.Sprintf("%v -> %s", ll.Value, ll.Next)
}

func main() {
	var ll *LinkedList
	ll = ll.Insert(0, 10)
	ll = ll.Insert(0, "foo")
	ll = ll.Insert(1, 4.5)
	ll = ll.Insert(10, false)
	ll = ll.Insert(2, struct{}{})
	fmt.Println(ll)
}
