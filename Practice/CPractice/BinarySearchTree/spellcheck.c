/*
 * spellcheck.c: a very simple spellchecker.
 * usage: spellcheck dictionary infile
 *        where dictionary is a dictionary file, one word per line
 *        infile - the input file to check
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "bst.h"
#define BUFSIZE 1000


int linecount = 1; // global variable.  Bad!!

/* Display usage message and quit */
static void usage(void) 
{
    fprintf(stderr, "usage: spellcheck dictionary infile\n");
    exit(EXIT_FAILURE);
}


/* Read dictionary into BST from nominated file.  Returns the BST.
 * If file cannot be opened, prints usage message and quits. 
 */
static BST *read_dictionary(char *filename)
{
    FILE *dictfile = fopen(filename, "r");
    if (!dictfile) {
        usage();
    }

    BST *dict = NULL;
    char wordbuf[BUFSIZE];

    while(!feof(dictfile)) {
        fgets(wordbuf, BUFSIZE, dictfile);
        int i;        
    for(i = 0; wordbuf[i] && wordbuf[i] != '\n'; i++); // do nothing loop
        wordbuf[i] = '\0';
        char * key = strdup(wordbuf);
        dict = bst_insert(dict, key, NULL);
    }

    fclose(dictfile);
    return dict;
}


/* note: this code is UNSAFE and BAD PRACTICE.  Figure out why! */
/* returns a newly allocated copy of the next word in the file.  
   If there is no next word, return NULL*/

char *get_next_word(FILE *inf)
{
    char wordbuf[BUFSIZE];
    int buf_ind = 0;
    int c;

    while((c = fgetc(inf)) != EOF) {
        if( c == '\n') {
            linecount++;
        }
        if (isalpha(c)) {
       		ungetc(c, inf);     
		break;
        }
    }
    
    if(feof(inf)) {
        return NULL;
    }

    else {
        while( (c = fgetc(inf)) != EOF) {
            if(isalpha(c)) {
                wordbuf[buf_ind++] = tolower(c);
            }
            else {
                ungetc(c, inf);
                break;
            }
        }

        wordbuf[buf_ind] = '\0';
        return strdup(wordbuf);   
     }
}


/* Checks the spelling of an input file against the dictionary. */
void check_spelling(FILE *infile, BST *dictionary)
{
    printf("Incorrect words:\n");
    char *word;    
    while((word = get_next_word(infile)) != NULL) {

        if(!bst_lookup(dictionary, word)) {
            printf("line %d: %s\n", linecount, word);
        }
        free(word);
    }

    return;
}


int main(int argc, const char *argv[]) 
{
    /* Validate command line arguments */
    if(argc != 3) usage();
    BST *dictionary = read_dictionary((char *) argv[1]);
    
    FILE *infile = fopen(argv[2], "r");

    if(infile && dictionary) {
        check_spelling(infile, dictionary);
    }
    else {
        usage();
    }

    return 0;
}


