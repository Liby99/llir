void *malloc(int size);

void free(void *ptr) {
  ptr = 0;
}

int f(int i, int j) {
  int sum = i + j;
  int *ptr = (int *) malloc(sum);
  ptr[0] = 10;
  free(ptr);
  return sum;
}