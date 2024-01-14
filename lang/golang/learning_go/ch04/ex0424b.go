package main

func main() {
loop:
	for i := 0; i < 10; i++ {
		switch {
		case i%2 == 0:
			println(i, ":偶数")
		case i%3 == 0:
			println(i, ":3で割り切れるが2では割り切れない")
		case i%7 == 0:
			println(i, ":ループ終了!")
			break loop
		default:
			println(i, ":退屈な数")
		}
	}
}
