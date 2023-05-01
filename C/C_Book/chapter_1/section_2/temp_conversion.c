#include <stdio.h>

void f_to_c() {
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

void c_to_f() {
	printf("Celsius -> Fahrenheit | Conversion Table\n\n\n");

	float fahr, celsius;
	int lower, upper, step;

	lower = 0; /* lower limit of temp table */
	upper = 300; /* upper limit of temp table */
	step = 20; /* step (iteration) size*/

	celsius = lower;
	while (celsius <= upper) {
		fahr = (celsius * (9.0 / 5.0)) + 32.0; /* calculate the celsius translation */

		printf("%3.0f %6.1f\n", celsius, fahr); /* %d to print integers */ 

		celsius = celsius + step; /* increase fahr by step size */
	}
}



int main() {
	f_to_c();
	printf("\n\n\n");
	c_to_f();
}
