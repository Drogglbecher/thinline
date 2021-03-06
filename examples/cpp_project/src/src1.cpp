#include <header1.hpp>

/**
 * \brief Function that returns a number.
 *
 * # TESTCASE(static_fct)
 * unsigned int no1 = 5;
 * # EQ[TL_FCT(no1: no1) => 5]
 */
static unsigned int static_fct(unsigned int no1) {
    return no1;
}

using namespace ns1;
using namespace ns2;

unsigned int c1::add_two_numbers(unsigned int no1, unsigned int no2) {
    return no1 + no2;
}

unsigned int c2::add_three_numbers(unsigned int no1, unsigned int no2, unsigned int no3) {
    return no1 + no2 + no3;
}

unsigned int c3::return5() {
    return 5;
}
