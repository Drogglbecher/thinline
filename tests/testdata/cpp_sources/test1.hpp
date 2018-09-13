#ifndef HEADER1_H_
#define HEADER1_H_

namespace ns1 {
    class c1 {
        public:
            /**
            * \brief Function that adds two numbers.
            *
            * #TL_TESTCASE(c1::AddTwoNumbers)
            *     unsigned int no1 = 5;
            *     #TL_EQ[this->class_inst->TL_FCT(no1: no1, no2: 10) => 15]
            *     #TL_LT[this->class_inst->TL_FCT(no1: no1, no2: 10) => 30]
            * #!TL_TESTCASE
            */

            unsigned int add_two_numbers(unsigned int no1, unsigned int no2);
            /**
            * \brief Class constructor.
            *
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
            c1() {};

            /**
            * \brief Class destructor.
            */
            ~c1() {};

    };

    class c2 {
        public:
            /**
            * \brief Class constructor.
            *
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
            c2() {};

            /**
            * \brief Class destructor.
            */
            ~c2() {};

            /**
            * \brief Function that adds two numbers.
            *
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
            c3() {};
            ~c3() {};
            unsigned int return5();
    };
}  // namespace ns2

#endif //HEADER1_H_
