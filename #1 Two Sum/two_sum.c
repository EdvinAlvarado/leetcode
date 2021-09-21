// Returned array must be malloced
// assume caller calls free()
int* twoSum(int* nums, int numSize, int target, int* returnSize) {
	int* results = (int*)calloc(2, sizeof(int));
	*returnSize = 2;

	for(int j = 0; j < numSize; j++) {
		for(int i = 0; i < numSize; i++) {
			if (i != j && target == nums[i] + nums[j]) {
				results[0] = j;
				results[1] = i;
				return results;
			}
		}
	}
	return results;
}
