#include <stdio.h>

void file_copying() {
	int c;

	// Excercise 1-7
	printf("%d", EOF);

	while ((c = getchar()) != EOF) {
		putchar(c);
	}
}

void character_count() {
	long nc;

	nc = 0;
	while(getchar() != EOF) {
		nc++;
	}
	printf("%ld\n", nc);
}

void line_count() {
	int c, nl;

	nl = 0;
	while ((c = getchar()) != EOF) {
		if (c == '\n') {
			nl++;
		}
	}

	printf("%d", nl);
}

int main() {
	// File Copying (1.5.1)
	//file_copying();
	
	// Character Count (1.5.2)
	//character_count();
	
	// Line Count (1.5.3)
	line_count();
}
