package main

import (
	"testing"
)

func TestAppName(t *testing.T) {
	expected := "Zoo Application"
	actual := AppName()

	if actual != expected {
		t.Errorf("%s != %s", actual, expected)
	}
}
