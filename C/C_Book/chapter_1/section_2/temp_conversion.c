#include <stdio.h>

int main() {
	printf("Fahrenheit -> Celsius | Conversion Table\n\n\n");

	float fahr, celsius;
	int lower, upper, step;

	lower = 0; /* lower limit of temp table */
	upper = 300; /* upper limit of temp table */
	step = 20; /* step (iteration) size*/

	fahr = lower;
	while (fahr <= upper) {
		celsius = (5.0 / 9.0) * (fahr - 32.0); /* calculate the celsius translation */

		printf("%3.0f %6.1f\n", fahr, celsius); /* %d to print integers */ 

		fahr = fahr + step; /* increase fahr by step size */
	}
}
