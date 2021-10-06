struct ListNode {
	int val;
	ListNode* next;
	ListNode() {val=0; next=nullptr;}
	ListNode(int x) {val=x; next=nullptr;}
	ListNode(int x, ListNode* new_node) {val=x; next=new_node;}
};

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
	ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
    	ListNode* p = l1; 
		ListNode* q = l2;
		ListNode head = ListNode();
		ListNode* curr = &head;
		int carry = 0;
		bool one = false;
		bool two = false;
		while (true) {
			int sum = carry;
			if (!one) {
				sum += p->val;
				if (p->next != nullptr) {p = p->next;} else {one = true;}
			}
			if (!two) {
				sum += q->val;
				if (q->next != nullptr) {q = q->next;} else {two = true;}
			}		
			carry = sum/10;
			curr->next = new ListNode(sum%10);
			curr = curr->next;
			if (one && two) {break;}
		}
	if (carry > 0) {curr->next = new ListNode(carry);}
	return head.next;	
	}
};
