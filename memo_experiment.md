# Tokyo
```
/home/cao/work/PCT/tools/trajectory_hash/data_copied/tokyo/client-24-22-49999.csv
PCT/tools/trajectory_hash/data_copied/tokyo/server-14000-24-22.csv 20160000
```
QUERY_SIZE=1439 ENCODEDVALUE_SIZE=8 FEATURE="fsa st" make

RUST_BACKTRACE=1 bin/app 1200000 tools/trajectory_hash/data/tokyo 49999 tools/trajectory_hash/data/tokyo/server-14000-24-22.csv
=>[UNTRUSTED] upload_query_data Failed SGX_ERROR_OUT_OF_MEMORY!

RUST_BACKTRACE=1 bin/app 1200000 tools/trajectory_hash/data/tokyo 100 tools/trajectory_hash/data/tokyo/server-14000-24-22.csv
=>init_enclave...
[SGX CLOCK] buffers initialize:  0.000040 seconds
[SGX CLOCK] reading:  0.004096 seconds
[SGX CLOCK] decrypt each queries:  0.006821 seconds
[SGX CLOCK] store queies:  0.004531 seconds
[SGX CLOCK] whole:  0.015644 seconds
[SGX CLOCK] central data decryption:  0.000853 seconds
[SGX CLOCK] central data decryption:  0.000329 seconds
[SGX CLOCK] central data decryption:  0.000321 seconds
[SGX CLOCK] central data decryption:  0.000180 seconds
[SGX CLOCK] central data decryption:  0.000186 seconds
[SGX CLOCK] central data decryption:  0.000219 seconds
[SGX CLOCK] central data decryption:  0.000294 seconds
[SGX CLOCK] central data decryption:  0.000140 seconds
[SGX CLOCK] central data decryption:  0.000202 seconds
[SGX CLOCK] central data decryption:  0.000533 seconds
[SGX CLOCK] central data decryption:  0.000568 seconds
[SGX CLOCK] central data decryption:  0.000401 seconds
[SGX CLOCK] central data decryption:  0.000873 seconds
[SGX CLOCK] central data decryption:  0.000178 seconds
[SGX CLOCK] central data decryption:  0.000245 seconds
[SGX CLOCK] central data decryption:  0.000525 seconds
[SGX CLOCK] central data decryption:  0.000650 seconds
positive result queryIds: [5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98]
[Clocker] Distribute central data:  7.003066 seconds
[Clocker] ECALL init_enclave:  2.699922 seconds
[Clocker] Read Central Data:  7.194077 seconds
[Clocker] ECALL upload_query_data:  0.019575 seconds
[Clocker] ECALL get_result:  0.000164 seconds
[Clocker] Read Query Data:  0.257938 seconds
[Clocker] ECALL private_contact_trace:  0.074392 seconds


RUST_BACKTRACE=1 bin/app 2300000 tools/trajectory_hash/data/tokyo 100 tools/trajectory_hash/data/tokyo/server-14000-24-22.csv
=>init_enclave...
[SGX CLOCK] buffers initialize:  0.000040 seconds
[SGX CLOCK] reading:  0.004102 seconds
[SGX CLOCK] decrypt each queries:  0.006874 seconds
[SGX CLOCK] store queies:  0.004312 seconds
[SGX CLOCK] whole:  0.015520 seconds
[SGX CLOCK] central data decryption:  0.001050 seconds
[SGX CLOCK] central data decryption:  0.000598 seconds
[SGX CLOCK] central data decryption:  0.000317 seconds
[SGX CLOCK] central data decryption:  0.000435 seconds
[SGX CLOCK] central data decryption:  0.000532 seconds
[SGX CLOCK] central data decryption:  0.000889 seconds
[SGX CLOCK] central data decryption:  0.001207 seconds
[SGX CLOCK] central data decryption:  0.000479 seconds
[SGX CLOCK] central data decryption:  0.001089 seconds
positive result queryIds: [5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98]
[Clocker] Read Central Data:  7.214839 seconds
[Clocker] Read Query Data:  0.256561 seconds
[Clocker] ECALL upload_query_data:  0.019474 seconds
[Clocker] ECALL private_contact_trace:  0.045881 seconds
[Clocker] ECALL get_result:  0.000154 seconds
[Clocker] Distribute central data:  7.092797 seconds
[Clocker] ECALL init_enclave:  2.703656 seconds



# Kinki
```
/home/cao/work/PCT_original/tools/trajectory_hash/data/kinki/client-24-22-49999.csv
/home/cao/work/PCT_original/tools/trajectory_hash/data/kinki/server-14000-24-22.csv
```

QUERY_SIZE=1439 ENCODEDVALUE_SIZE=8 FEATURE="fsa st" make

RUST_BACKTRACE=1 bin/app 2300000 tools/trajectory_hash/data/kinki 100 tools/trajectory_hash/data/kinki/server-14000-24-22.csv


# 21-21
```
/home/cao/work/PCT_original/tools/trajectory_hash/data/21-21/client-21-21-0.csv 2016000
/home/cao/work/PCT_original/tools/trajectory_hash/data/21-21/server-1000-21-21.csv
/home/cao/work/PCT_original/tools/trajectory_hash/data/21-21/server-5000-21-21.csv
```
QUERY_SIZE=20160 ENCODEDVALUE_SIZE=8 FEATURE="fsa st" make
