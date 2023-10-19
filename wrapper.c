#include <acfutils/glew.h>


void glew_init(void) {
    glewInit();
}

void glew_dllmain_hook(unsigned long reason) {
    lacf_glew_dllmain_hook(reason);
}
