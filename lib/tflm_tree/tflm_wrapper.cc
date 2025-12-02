
#include "tensorflow/lite/micro/micro_interpreter.h"
#include "tensorflow/lite/micro/micro_mutable_op_resolver.h"
#include "tensorflow/lite/schema/schema_generated.h"
//#include "tensorflow/lite/version.h"
extern "C" bool __cxa_guard_acquire() { return 1; }
extern "C" void __cxa_guard_release() {}
extern "C" void __cxa_guard_abort() {}
extern "C" void __throw_bad_alloc() {
  while (1)
    ;
}
extern "C" void __throw_length_error() {
  while (1)
    ;
}

extern "C" void __aeabi_atexit() {}
extern "C" void __cxa_pure_virtual() {
  while (1)
    ;
}
extern "C" int strcmp(const char* s1, const char* s2) {
  while (*s1 && (*s1 == *s2)) {
    s1++;
    s2++;
  }
  return *(unsigned char*)s1 - *(unsigned char*)s2;
}
extern "C" void __assert_func(const char* file, int line, const char* func,
                              const char* failedexpr) {
  while (1)
    ;
}

extern "C" void abort() {
  while (true) {
    __asm__("bkpt #0");
  }
}

extern "C" {
int vsnprintf(char* str, size_t size, const char* format, va_list ap) {
  return 0;
}

int strncmp(const char* s1, const char* s2, size_t n) {
  while (n-- && *s1 && (*s1 == *s2)) {
    s1++;
    s2++;
  }
  return (n == 0) ? 0 : (*s1 - *s2);
}

double frexp(double x, int* exp) {
  *exp = 0;
  return x;
}
}

// Arena dla STM32F4
constexpr int kTensorArenaSize = 60 * 1024;
static uint8_t tensor_arena[kTensorArenaSize];

// Globalne obiekty
static tflite::MicroInterpreter* interpreter = nullptr;
static TfLiteTensor* input_tensor = nullptr;
static TfLiteTensor* output_tensor = nullptr;

extern "C" {

// Inicjalizacja modelu

int tflm_init(const uint8_t* model_data, size_t model_size) {
  static tflite::MicroMutableOpResolver<3> resolver;
  resolver.AddFullyConnected();
  resolver.AddRelu();
  resolver.AddSoftmax();

  const tflite::Model* model = tflite::GetModel(model_data);
  // if (model->version() != TFLITE_SCHEMA_VERSION) return -1;

  static tflite::MicroInterpreter static_interpreter(
      model, resolver, tensor_arena, kTensorArenaSize);
  interpreter = &static_interpreter;

  if (interpreter->AllocateTensors() != kTfLiteOk) return -2;

  input_tensor = interpreter->input(0);
  output_tensor = interpreter->output(0);
  return 0;
}

// Ustaw dane wejściowe
void tflm_set_input(const float* data, int len) {
  for (int i = 0; i < len; i++) {
    input_tensor->data.f[i] = data[i];
  }
}

// Uruchom inferencję
int tflm_invoke() { return interpreter->Invoke() == kTfLiteOk ? 0 : -1; }

// Pobierz dane wyjściowe
void tflm_get_output(float* out, int len) {
  for (int i = 0; i < len; i++) {
    out[i] = output_tensor->data.f[i];
  }
}
}
