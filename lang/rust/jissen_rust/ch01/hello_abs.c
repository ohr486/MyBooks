#include <stdio.h>

int my_abs(int n)
{
	int res = (n >= 0) ? n : -n;
}

int main(int argc, char *argv[])
{
	int n = 10;
	int n_abs = my_abs(n);

	printf("n = %d, my_abs(n) = %d\n", n, n_abs);
	return 0;
}

