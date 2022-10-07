// Generated by gir (https://github.com/gtk-rs/gir @ b5068ede6c51)
// from gir-files (https://github.com/gtk-rs/gir-files @ 01c4ec663b3f)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ 40cea11af0f3)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) GST_APP_LEAKY_TYPE_DOWNSTREAM);
    PRINT_CONSTANT((gint) GST_APP_LEAKY_TYPE_NONE);
    PRINT_CONSTANT((gint) GST_APP_LEAKY_TYPE_UPSTREAM);
    PRINT_CONSTANT((gint) GST_APP_STREAM_TYPE_RANDOM_ACCESS);
    PRINT_CONSTANT((gint) GST_APP_STREAM_TYPE_SEEKABLE);
    PRINT_CONSTANT((gint) GST_APP_STREAM_TYPE_STREAM);
    return 0;
}
