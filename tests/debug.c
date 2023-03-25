#include <stdio.h>
#include <unistd.h>

int main (int argc, char *argv[]) {
  printf("C program pid: %d\n", getpid());
  printf("Hello world\n");
  return 0;
}
