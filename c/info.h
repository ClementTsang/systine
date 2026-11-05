#ifndef INFO_H_
#define INFO_H_

typedef struct LoadAvg {
        double one;
        double five;
        double fifteen;
} LoadAvg;

typedef struct MemInfo {
        unsigned long long total;
        unsigned long long free;
        unsigned long long avail;

        unsigned long long buffers;
        unsigned long long cached;

        unsigned long long swap_total;
        unsigned long long swap_free;
} MemInfo;

typedef struct DiskInfo {
        unsigned long long total;
        unsigned long long free;
} DiskInfo;

const char *get_os_type(void);
const char *get_os_release(void);

#include <stdint.h>

unsigned int get_cpu_num(void);
uint64_t get_cpu_speed(void);

LoadAvg get_loadavg(void);
uint64_t get_proc_total(void);

MemInfo get_mem_info(void);
DiskInfo get_disk_info(void);

#endif
  
