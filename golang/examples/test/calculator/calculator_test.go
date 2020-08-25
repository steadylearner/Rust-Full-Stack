package calculator

import (
	"testing"
)

// TestAdd as it is.
func TestAdd(t *testing.T) {

	expected := 18

	if got := Add(10, 8); got != expected {
		t.Errorf("expected %d. Got %d instead", expected, got)
	}

}

// TestMultiply as it is.
func TestMultiply(t *testing.T) {
	expected := 100

	if got := Multiply(10, 10); got != expected {
		t.Errorf("Expected %d. Got %d", expected, got)
	}
}