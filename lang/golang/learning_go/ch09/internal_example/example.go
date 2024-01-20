package internal_package_example

import "ohr486.net/internal_package_example/foo/internal"

func failUseDoubler(a int) int {
	return internal.Doubler(a)
}
