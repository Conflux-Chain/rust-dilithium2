#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include "../randombytes.h"
#include "../sign.h"

#define MLEN 59
#define NTESTS 1

void print_uint8(uint8_t *m, int length){
  for (int j = 0; j < length; ++j) {
    printf("%u ", m[j]);
  }
}

int main(void)
{
  unsigned int i, j;
  int ret;
  size_t mlen, smlen;
  uint8_t m[MLEN] = {0};
  uint8_t sm[MLEN + CRYPTO_BYTES];
  uint8_t m2[MLEN + CRYPTO_BYTES];
  uint8_t pk[CRYPTO_PUBLICKEYBYTES];
  uint8_t sk[CRYPTO_SECRETKEYBYTES];


  for(i = 0; i < NTESTS; ++i) {
    randombytes(m, MLEN);
    print_uint8(m, MLEN);

    crypto_sign_keypair(pk, sk);
    // printf("\n\n pk:");
    // print_uint8(pk, CRYPTO_PUBLICKEYBYTES);
    
    // printf("\n\n sk:");
    // print_uint8(sk, CRYPTO_SECRETKEYBYTES);

    crypto_sign(sm, &smlen, m, MLEN, sk);
    // printf("\n\n sm:");
    // print_uint8(sm, MLEN + CRYPTO_BYTES);

    // verify signature
    printf("\nmlen:%lu\n",mlen);
    printf("\nsmlen:%lu\n",smlen);

    ret = crypto_sign_open(m2, &mlen, sm, smlen, pk);
    printf("\n\n m2:\n");
    print_uint8(m2, MLEN);

    if(ret) {
      fprintf(stderr, "Verification failed\n");
      return -1;
    }

    if(mlen != MLEN) {
      fprintf(stderr, "Message lengths don't match\n");
      return -1;
    }

    for(j = 0; j < mlen; ++j) {
      if(m[j] != m2[j]) {
        fprintf(stderr, "Messages don't match\n");
        return -1;
      }
    }

    /*
    randombytes((uint8_t *)&j, sizeof(j));
    do {
      randombytes(m2, 1);
    } while(!m2[0]);
    sm[j % CRYPTO_BYTES] += m2[0];
    ret = crypto_sign_open(m2, &mlen, sm, smlen, pk);
    if(!ret) {
      fprintf(stderr, "Trivial forgeries possible\n");
      return -1;
    }  
    
    */

  }

  printf("\nCRYPTO_PUBLICKEYBYTES = %d\n", CRYPTO_PUBLICKEYBYTES);
  printf("CRYPTO_SECRETKEYBYTES = %d\n", CRYPTO_SECRETKEYBYTES);
  printf("CRYPTO_BYTES = %d\n", CRYPTO_BYTES);

  return 0;
}
