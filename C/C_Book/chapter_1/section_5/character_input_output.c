#include <stdio.h>


int main() {
	int c;

	// Excercise 1-7
	printf("%d", EOF);

	while ((c = getchar()) != EOF) {
		putchar(c);
	}
}
