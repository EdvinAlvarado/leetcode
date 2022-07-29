#include <stdio.h> // sprintf
#include <stdbool.h> // bool
#include <stdlib.h> // atoi
#include <string.h> // strlen

bool isPalindrome(int x) {
	if (x < 0) {return false;}
	char str[12];
	sprintf(str, "%d", x);
	bool ret = true;
	for (unsigned int i = 0; i < strlen(str); i++) {
		ret = ret && (str[strlen(str) - i - 1] == str[i]);
	}
	return ret;
}

int main() {
	bool b = isPalindrome(100);
	printf("%u", b);
}
