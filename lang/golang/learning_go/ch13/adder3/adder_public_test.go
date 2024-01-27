package adder_test

import (
	"ohr486.net/test_example/adder3"
	"testing"
)

func TestAddNumbers(t *testing.T) {
	result := adder.AddNumbers(2, 3)

	if result != 5 {
		t.Error("結果が正しくない: 想定される結果 5, 得られた結果", result)
	}
}
