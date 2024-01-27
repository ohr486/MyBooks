package adder

import (
	"fmt"
	"os"
	"testing"
	"time"
)

var testTime time.Time

func TestMain(m *testing.M) {
	fmt.Println("テストの準備")
	testTime = time.Now()
	exitVal := m.Run()
	fmt.Println("テストの後の後片付け")
	os.Exit(exitVal)
}

func TestFirst(t *testing.T) {
	fmt.Println("TestFirstではTestMainで設定されたものを使う:", testTime)
}

func TestSecond(t *testing.T) {
	fmt.Println("TestSecondでもTestMainで設定されたものを使う:", testTime)
}
