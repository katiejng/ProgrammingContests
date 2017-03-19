#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* 
 * structure and functions to represent a binary search tree
 * by convention, an empty tree is represented by a NULL pointer
 * at all times, we maintain the invariant that all keys
 * in the left node  are "smaller" than the 
 * current key, and all keys in the right node are larger.  
 * There should be no duplicate keys. 

 */
 
/* KVP represents a key-value pair in a binary search tree*/ 
typedef struct {
	char *key;
	void *value;
} KVP;

/* BST represents a node of the binary search tree */
typedef struct bst {
	struct bst *left;
	struct bst *right;
	KVP kvp;
} BST;


/*
 * insert a key/value pair into the BST
 * only copies the pointers so you should make a copy before
 * insertion if they are subject to change
 * returns: the updated BST.
 */
BST *bst_insert(BST *bst, char *key, void *value){
    
    //iterate through BST
    //if root has the same key, set the value to value
    //if key < bst.key go left. if left is none, create a bst, kvp
    //if key > bst.key go right. if right is none, create a bst, kvp
    
    
    if (bst==NULL){
        bst = (BST * )malloc(sizeof(BST));
        
        bst->kvp.key = key;
        bst->kvp.value = value;
        bst->left = NULL;
        bst->right = NULL;
        return bst;
    }
    
    

    int myVal = strcmp(bst->kvp.key,key); 
    if (myVal==0){
        bst->kvp.value = 0;
    }else if(myVal<0){
        bst->left = bst_insert(bst->left, key, value);
    }else if(myVal>0){
        bst->right = bst_insert(bst->right, key, value);
    }else{

        fprintf(stderr, "something is wrong\n");

    }
    return bst;
}

/*
 * returns a key/value pair from the BST
 * returns null if the key is not present
 */
KVP *bst_lookup(BST *bst, char *key){
    if (bst==NULL){
        return NULL;
    }
    /*
    printf("\nfail: %s %s", bst->kvp.key, key);
    printf("\nleft: %s", bst->left);
    printf("\nright: %s", bst->right);
    */

    int myVal = strcmp(bst->kvp.key,key);

    if (myVal==0){
        KVP*kvpptr;
        kvpptr = &(bst->kvp);
        return kvpptr;
    }else if(myVal<0){
        return bst_lookup(bst->left, key);
    }else if(myVal>0){    
        return bst_lookup(bst->right, key);
    }else{

        fprintf(stderr, "something is wrong\n");
    }
    
    
    
    return NULL;
}





