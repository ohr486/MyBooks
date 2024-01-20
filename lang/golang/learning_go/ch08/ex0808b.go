package main

import (
	"errors"
	"fmt"
	"reflect"
)

type MyErr struct {
	Codes []int
}

func (me MyErr) Error() string {
	return fmt.Sprintf("codes: %v", me.Codes)
}

func (me MyErr) Is(target error) bool {
	if me2, ok := target.(MyErr); ok {
		return reflect.DeepEqual(me, me2)
	}
	return false
}

func (me MyErr) Code() []int {
	return me.Codes
}

func main() {
	err := AFunctionThatReturnAnError()
	var myErr MyErr
	if errors.As(err, &myErr) {
		fmt.Println(myErr.Codes)
	}

	err = AFunctionThatReturnAnError()
	var coder interface {
		Code() []int
	}
	if errors.As(err, &coder) {
		fmt.Println(coder.Code())
	}
}

func AFunctionThatReturnAnError() error {
	return errors.New("my error")
	// return MyErr{Codes: []int{404, 501}}
}
