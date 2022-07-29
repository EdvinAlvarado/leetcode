#include <stdio.h> // sprintf
#include <stdbool.h> // bool
#include <stdlib.h> // atoi
#include <string.h> // strlen

bool isPalindrome(int x) {
	if (x < 0) {return false;}
	char str[12];
	sprintf(str, "%d", x);
	char rev[12];
	for (unsigned int i = 0; i < strlen(str); i++) {
		rev[strlen(str) - i - 1] = str[i];
	}
	int rev_x = atoi(rev);
	return rev_x == x;
}

int main() {
	bool b = isPalindrome(12);
	printf("%u", b);
}
