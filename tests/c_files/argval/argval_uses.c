void kfree(void *ptr);

struct Object {
    int i;
};

// This tests argument used after function call
void run1(int i, struct Object* obj) {
    kfree(obj);
    if (i == 0) {
        obj->i = 10;
    }
}

// This tests argument used after call in a function call
void run2(int i, struct Object* obj) {
    kfree(obj);
    if (i == 0) {
        kfree(obj);
    }
}