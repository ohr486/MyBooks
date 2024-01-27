package table

import "testing"

func TestDoMath(t *testing.T) {
	result, err := DoMath(2, 2, "+")
	if result != 4 {
		t.Error("期待する結果 4, 得られた結果", result)
	}
	if err != nil {
		t.Error("予想されたエラー nil, 得られたエラー", err)
	}

	result2, err2 := DoMath(2, 2, "-")
	if result2 != 0 {
		t.Error("期待する結果 4, 得られた結果", result)
	}
	if err2 != nil {
		t.Error("予想されたエラー nil, 得られたエラー", err)
	}

	result3, err3 := DoMath(2, 2, "*")
	if result3 != 4 {
		t.Error("期待する結果 4, 得られた結果", result)
	}
	if err3 != nil {
		t.Error("予想されたエラー nil, 得られたエラー", err)
	}

	result4, err4 := DoMath(2, 2, "/")
	if result4 != 1 {
		t.Error("期待する結果 4, 得られた結果", result)
	}
	if err4 != nil {
		t.Error("予想されたエラー nil, 得られたエラー", err)
	}
}

func TestDoMathTable(t *testing.T) {
	data := []struct {
		name     string
		num1     int
		num2     int
		op       string
		expected int
		errMsg   string
	}{
		{"addition", 2, 2, "+", 4, ""},
		{"subtraction", 2, 2, "-", 0, ""},
		{"multiplication", 2, 2, "*", 4, ""},
		{"division", 2, 2, "/", 1, ""},
		{"bad_division", 2, 0, "/", 0, `0による除算`},
		{"bad_op", 2, 2, "?", 0, `未知の演算子 ?`},
	}

	for _, d := range data {
		t.Run(d.name, func(t *testing.T) {
			result, err := DoMath(d.num1, d.num2, d.op)
			if result != d.expected {
				t.Errorf("期待する結果 %d, 得られた結果 %d", d.expected, result)
			}
			var errMsg string
			if err != nil {
				errMsg = err.Error()
			}
			if errMsg != d.errMsg {
				t.Errorf("予想されたエラーメッセージ `%s`, 得られたエラーメッセージ `%s`", d.errMsg, errMsg)
			}
		})
	}
}
