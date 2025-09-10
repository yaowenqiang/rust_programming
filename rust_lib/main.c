#include <stdio.h>
#include "rust_lib.h"

int main() {
    // 调用整数加法函数
    int result = add(3, 7);
    printf("3 + 7 = %d\n", result);

    // 调用字符串处理函数
    const char* name = "World";
    const char* greeting = greet(name);
    printf("%s\n", greeting);
    
    // 释放Rust分配的内存
    free_string((char*)greeting);
    
    return 0;
}
