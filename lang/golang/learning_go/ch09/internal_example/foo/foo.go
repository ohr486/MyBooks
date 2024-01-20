package foo

import "ohr486.net/internal_package_example/foo/internal"

func UseDoubler(i int) int {
	return internal.Doubler(i)
}
