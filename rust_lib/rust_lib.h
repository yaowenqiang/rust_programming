#ifdef _WIN32
#define EXPORT __declspec(dllimport)
#else
#define EXPORT extern
#endif

EXPORT int add(int a, int b);
EXPORT const char* greet(const char* name);
EXPORT void free_string(char* s);
