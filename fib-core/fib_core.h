#ifndef FIB_CORE_H
#define FIB_CORE_H

#include <stdint.h>

int32_t fib_number(uint64_t n, uint64_t *out);
int32_t fib_sequence(uint64_t n, uint64_t *out, uint64_t len);

#endif /* FIB_CORE_H */
