package mock

//go:generate mockgen -source=foo.go -destination=./test/test_helper/foo_mock.go -package=test_helper
type Foo interface {
	Bar(x int) int
	Set(s string) string
}

type FooImpl struct {
}

func (*FooImpl) Bar(x int) int {
	return x
}
