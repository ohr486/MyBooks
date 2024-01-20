package sibling

import "ohr486.net/internal_package_example/foo/internal"

func AlsoUseDoubler(i int) int {
	return internal.Doubler(i)
}
