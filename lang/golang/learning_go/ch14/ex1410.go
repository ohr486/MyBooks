package main

import (
	"errors"
	"fmt"
	"reflect"
	"time"
)

type outExp struct {
	out    []reflect.Value
	expiry time.Time
}

func buildInStruct(ft reflect.Type) (reflect.Type, error) {
	if ft.NumIn() == 0 {
		return nil, errors.New("must have at least one param")
	}

	sf := make([]reflect.StructField, 0, ft.NumIn())
	for i := 0; i < ft.NumIn(); i++ {
		ct := ft.In(i)
		if !ct.Comparable() {
			return nil, fmt.Errorf("parameter %d of type %s and kind %v is not comparable", i+1, ct.Name(), ct.Kind())
		}
		sf = append(sf, reflect.StructField{
			Name: fmt.Sprintf("F%d", i),
			Type: ct,
		})
	}
	s := reflect.StructOf(sf)
	return s, nil
}

func Memoizer(f any, expiration time.Duration) (any, error) {
	ft := reflect.TypeOf(f)
	if ft.Kind() != reflect.Func {
		return nil, errors.New("only for functions")
	}

	inType, err := buildInStruct(ft)
	if err != nil {
		return nil, err
	}

	if ft.NumOut() == 0 {
		return nil, errors.New("must have at least one returned value")
	}

	m := map[any]outExp{}
	fv := reflect.ValueOf(f)

	memo := reflect.MakeFunc(ft, func(args []reflect.Value) []reflect.Value {
		iv := reflect.New(inType).Elem()
		for k, v := range args {
			iv.Field(k).Set(v)
		}
		ivv := iv.Interface()

		ov, ok := m[ivv]
		now := time.Now()
		if !ok || ov.expiry.Before(now) {
			ov.out = fv.Call(args)
			ov.expiry = now.Add(expiration)
			m[ivv] = ov
		}
		return ov.out
	})
	return memo.Interface(), nil
}

func AddSlowly(a, b int) int {
	time.Sleep(100 * time.Millisecond)
	return a + b
}

func main() {
	addSlowlyInterface, err := Memoizer(AddSlowly, 2*time.Second)
	if err != nil {
		panic(err)
	}

	addSlowly := addSlowlyInterface.(func(int, int) int)
	for i := 0; i < 5; i++ {
		start := time.Now()
		result := addSlowly(1, 2)
		end := time.Now()
		fmt.Printf("結果(%d):%d 所要時間:%v\n", i, result, end.Sub(start))
	}
	time.Sleep(3 * time.Second)
	start := time.Now()
	result := addSlowly(1, 2)
	end := time.Now()
	fmt.Printf("結果:%d 所要時間:%v(制限時間経過後)\n", result, end.Sub(start))
}
