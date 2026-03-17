#include <gtk/gtk.h>
#include "ide.h"

int main(int argc, char *argv[]) {
    gtk_init();
    
    ProtlinTJ::IDE ide;
    ide.run();
    
    return 0;
}