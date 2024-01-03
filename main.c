#include <stdlib.h>

char *fold()
{
  int bytes[] = {97, 97, 97, 97, /*___*/ 98, 98, 98, 98, 98, 98, /*___*/ 99, /*___*/ 100, 100, 100, 100, 100};
  size_t length_divided = sizeof(bytes) / sizeof(bytes[0]);

  char *result = malloc(length_divided + 1);
  if (!result)
  {
    return NULL;
  }

  for (size_t i = 0; i < length_divided; i++)
  {
    result[i] = (char)bytes[i];
  }

  result[length_divided] = '\0';

  return result;
}

int main()
{
  char *result = fold();
  if (result)
  {
    printf("%s\n", result);
    free(result);
  }
  return 0;
}