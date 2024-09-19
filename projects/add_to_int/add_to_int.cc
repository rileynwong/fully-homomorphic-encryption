#include "add_to_int.h"

#pragma hls_top
int add_to_int(int state, int int_to_add){
    return state + int_to_add;
}
