package internal_package_example

import "ohr486.net/internal_package_example/foo/internal"

func FailUseDoubler(i int) int {
	return internal.Doubler(i)
}
