#ifndef RVBL_ASSERT_H
#define RVBL_ASSERT_H

#define assert(x)                                                                                  \
    if (!(x)) {                                                                                    \
        __asm__ volatile("ebreak");                                                                \
    } else {                                                                                       \
    }

#endif