#include <stdio.h>

int main() {
	int fahr, celsius;
	int lower, upper, step;

	lower = 0; /* lower limit of temp table */
	upper = 300; /* upper limit of temp table */
	step = 20; /* step (iteration) size*/

	fahr = lower;
	while (fahr <= upper) {
		celsius = 5 * (fahr - 32) / 9; /* calculate the celsius translation */

		printf("%d\t%d\n", fahr, celsius); /* %d to print integers */ 

		fahr = fahr + step; /* increase fahr by step size */
	}
}
