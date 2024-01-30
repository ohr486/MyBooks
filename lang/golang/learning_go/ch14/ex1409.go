package main

import (
	"fmt"
	"reflect"
	"runtime"
	"time"
)

func MakeTimeFunction(f any) any {
	rf := reflect.TypeOf(f)
	if rf.Kind() != reflect.Func {
		panic("関数を指定してください")
	}
	vf := reflect.ValueOf(f)
	wrapperF := reflect.MakeFunc(rf,
		func(in []reflect.Value) []reflect.Value {
			start := time.Now()
			out := vf.Call(in)
			end := time.Now()
			fmt.Printf("%sの所要時間: %v\n", runtime.FuncForPC(vf.Pointer()).Name(), end.Sub(start))
			return out
		})
	return wrapperF.Interface()
}

func timeMe() {
	time.Sleep(1 * time.Second)
}

func timeMeToo(a int) int {
	time.Sleep(time.Duration(a) * time.Second)
	result := a * 2
	return result
}

func main() {
	timed := MakeTimeFunction(timeMe).(func())
	timed()
	timedToo := MakeTimeFunction(timeMeToo).(func(int) int)
	fmt.Println("timedToo(2):", timedToo(2))
}
