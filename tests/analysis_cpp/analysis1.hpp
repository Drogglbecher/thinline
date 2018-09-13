#ifndef HEADER1_H_
#define HEADER1_H_

namespace ns1 {
    /**
     * #TL_TESTCLASS(c1)
     *     #TL_SET_UP_CONTEXT:
     *         this->class_inst = new c1();
     *     #TL_TEAR_DOWN_CONTEXT:
     *         delete this->class_inst;
     *         this->class_inst = nullptr;
     *     #TL_CLASS_CONTEXT:
     *         c1 * class_inst;
     * #!TL_TESTCLASS
     */
    class c1 {
        public:
            /**
             * #TL_TESTCASE(c1::AddTwoNumbers)
             *     unsigned int no1 = 5;
             *     #TL_EQ[this->class_inst->TL_FCT(no1: no1, no2: 10) => 15]
             *     #TL_LT[this->class_inst->TL_FCT(no1: no1, no2: 10) => 30]
             * #!TL_TESTCASE
             */
            unsigned int add_two_numbers(unsigned int no1, unsigned int no2);

            c1() = default;
            ~c1() = default;

    };

    /**
    * #TL_TESTCLASS(c2)
    *     #TL_SET_UP_CONTEXT:
    *         this->class_inst = new c2();
    *     #TL_TEAR_DOWN_CONTEXT:
    *         delete this->class_inst;
    *         this->class_inst = nullptr;
    *     #TL_CLASS_CONTEXT:
    *         c2 * class_inst;
    * #!TL_TESTCLASS
    */
    class c2 {
        public:
            c2() = default;
            ~c2() = default;

            /**
            * #TL_TESTCASE(c2::AddThreeNumbers)
            *     unsigned int no1 = 5;
            *     unsigned int no2 = 10;
            *     #TL_EQ[this->class_inst->TL_FCT(no1: no1, no2: no2, no3: 5) => 20]
            * #!TL_TESTCASE
            */
            unsigned int add_three_numbers(unsigned int no1, unsigned int no2, unsigned int no3);
    };
}  // namespace ns1

namespace ns2 {
    class c3 {
        public:
            c3() = default;
            ~c3() = default;

            unsigned int return5();
    };
}  // namespace ns2

#endif //HEADER1_H_
