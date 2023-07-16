package mock

import (
	"testing"

	"github.com/golang/mock/gomock"
	"github.com/stretchr/testify/assert"
)


func TestFoo(t *testing.T) {
	ctrl := gomock.NewController(t)

	// Assert that Bar() is invoked.
	defer ctrl.Finish()

	m := NewMockFoo(ctrl)

	// Asserts that the first and only call to Bar() is passed 99.
	// Anything else will fail.
	m.EXPECT().Bar(99).Return(99).AnyTimes()
	fooImpl := &FooImpl{}
	assert.Equal(t, 101, fooImpl.Bar(99))
}
