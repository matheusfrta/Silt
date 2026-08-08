#ifndef SILT_H
#define SILT_H

#ifdef __cplusplus
extern "C" {
#endif

void* silt_sig_new_f64(double v);
double silt_sig_get_f64(void* ptr);
void silt_sig_set_f64(void* ptr, double v);
void silt_sig_free_f64(void* ptr);

#ifdef __cplusplus
}
#endif

#endif