#include <stdarg.h>
#include <stdio.h>

typedef void (*callback_t)(void* v, size_t len, const char* buf);

static callback_t _print_callback;
static callback_t _err_callback;

void shim_set_print_callback(callback_t cb) {
	_print_callback = cb;
}

void shim_set_err_callback(callback_t cb) {
	_err_callback = cb;
}

void shim_print_fn(void* v, const char* s, ...) {
	va_list varargs;
	va_start(varargs, s);

	char* buf = NULL;
	size_t len = vsnprintf(buf, 0, s, varargs);
	
	buf = (char*) malloc(len + 1); // Add one for trailing null
	vsnprintf(buf, len, s, varargs);
	
	_print_callback(v, len, buf);

	free(buf);
	
	va_end(varargs);
}

void shim_err_fn(void* v, const char* s, ...) {
	va_list varargs;
	va_start(varargs, s);

	char* buf = NULL;
	size_t len = vsnprintf(buf, 0, s, varargs);
	buf = (char*) malloc(len + 1); // Add one for trailing null
	vsnprintf(buf, len, s, varargs);
	
	_err_callback(v, len, buf);

	free(buf);
	
	va_end(varargs);
}