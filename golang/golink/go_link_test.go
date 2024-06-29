package golink

import "testing"
import _ "unsafe"

// go:linkname privateRef demo.privateFunc
func privateRef()

func TestGoLink(t *testing.T) {
	privateRef()
}
