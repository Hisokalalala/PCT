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

RUST_BACKTRACE=1 bin/app 10000 tools/trajectory_hash/data_expanded/tokyo 100 tools/trajectory_hash/data_expanded/tokyo/server-14000-24-22.csv
=>
positive result queryIds: [(5, 0.0038400006), (6, 0.0038400006), (7, 0.0038400006), (8, 0.0038400006), (9, 0.0038400006), (10, 0.0038400006), (11, 0.0038400006), (12, 0.0038400006), (13, 0.0038400006), (14, 0.0038400006), (15, 0.0038400006), (16, 0.0038400006), (17, 0.0038400006), (18, 0.0038400006), (19, 0.0038400006), (20, 0.0038400006), (21, 0.0038400006), (22, 0.0038400006), (23, 0.0038400006), (24, 0.0038400006), (25, 0.0038400006), (26, 0.0038400006), (27, 0.0038400006), (28, 0.0038400006), (29, 0.0038400006), (30, 0.0038400006), (31, 0.0038400006), (32, 0.0038400006), (33, 0.0038400006), (34, 0.0038400006), (35, 0.0038400006), (36, 0.0038400006), (37, 0.0038400006), (38, 0.0038400006), (39, 0.0038400006), (40, 0.0038400006), (41, 0.0038400006), (42, 0.0038400006), (43, 0.0038400006), (44, 0.0038400006), (45, 0.0038400006), (46, 0.0038400006), (47, 0.0038400006), (48, 0.0038400006), (49, 0.0038400006), (50, 0.0038400006), (51, 0.0038400006), (52, 0.0038400006), (53, 0.0038400006), (54, 0.0038400006), (55, 0.0038400006), (56, 0.0038400006), (57, 0.0038400006), (58, 0.0038400006), (59, 0.0038400006), (60, 0.0038400006), (61, 0.0038400006), (62, 0.0038400006), (63, 0.0038400006), (64, 0.0038400006), (65, 0.0038400006), (66, 0.0038400006), (67, 0.0038400006), (68, 0.0038400006), (69, 0.0038400006), (70, 0.0038400006), (71, 0.0038400006), (72, 0.0038400006), (73, 0.0038400006), (74, 0.0038400006), (75, 0.0038400006), (76, 0.0038400006), (77, 0.0038400006), (78, 0.0038400006), (79, 0.0038400006), (80, 0.0038400006), (81, 0.0038400006), (82, 0.0038400006), (83, 0.0038400006), (84, 0.0038400006), (85, 0.0038400006), (86, 0.0038400006), (87, 0.0038400006), (88, 0.0038400006), (89, 0.0038400006), (90, 0.0038400006), (91, 0.0038400006), (92, 0.0038400006), (93, 0.0038400006), (94, 0.0038400006), (95, 0.0038400006), (96, 0.0038400006), (97, 0.0038400006), (98, 0.0038400006)]
[Clocker] ECALL init_enclave:  3.057511 seconds
[Clocker] ECALL upload_query_data:  0.029668 seconds
[Clocker] Read Central Data:  21.502283 seconds
[Clocker] ECALL private_contact_trace:  6.535682 seconds
[Clocker] Distribute central data:  9.554316 seconds
[Clocker] ECALL get_result:  0.228885 seconds
[Clocker] Read Query Data:  0.163887 seconds


RUST_BACKTRACE=1 bin/app 140000 tools/trajectory_hash/data_expanded/tokyo 100 tools/trajectory_hash/data_expanded/tokyo/server-14000-24-22.csv
=>
[Clocker] Read Central Data:  21.600356 seconds
[Clocker] ECALL init_enclave:  3.109033 seconds
[Clocker] Read Query Data:  0.160274 seconds
[Clocker] ECALL upload_query_data:  0.029891 seconds
[Clocker] Distribute central data:  9.040531 seconds
[Clocker] ECALL private_contact_trace:  0.565190 seconds
[Clocker] ECALL get_result:  0.216284 seconds


root@c624d50ecaa7:~/sgx/samplecode/PCT# RUST_BACKTRACE=1 bin/app 1200000 tools/trajectory_hash/data_expanded/tokyo 100 tools/trajectory_hash/data_expanded/tokyo/server-14000-24-22.csv
=>
[Clocker] ECALL upload_query_data:  0.029756 seconds
[Clocker] ECALL get_result:  0.213897 seconds
[Clocker] Distribute central data:  8.861293 seconds
[Clocker] Read Central Data:  21.747926 seconds
[Clocker] Read Query Data:  0.161211 seconds
[Clocker] ECALL private_contact_trace:  0.189240 seconds
[Clocker] ECALL init_enclave:  3.117446 seconds


RUST_BACKTRACE=1 bin/app 1200000 tools/trajectory_hash/data_expanded/tokyo 1000 tools/trajectory_hash/data_expanded/tokyo/server-14000-24-22.csv
=>
[Clocker] ECALL get_result:  0.218642 seconds
[Clocker] Distribute central data:  8.849062 seconds
[Clocker] ECALL init_enclave:  3.120117 seconds
[Clocker] ECALL private_contact_trace:  0.188842 seconds
[Clocker] Read Central Data:  21.597284 seconds
[Clocker] Read Query Data:  0.161736 seconds
[Clocker] ECALL upload_query_data:  0.030430 seconds


RUST_BACKTRACE=1 bin/app 1200000 tools/trajectory_hash/data_expanded/tokyo 10000 tools/trajectory_hash/data_expanded/tokyo/server-14000-24-22.csv
=>
[Clocker] Read Query Data:  0.164956 seconds
[Clocker] Read Central Data:  21.533509 seconds
[Clocker] ECALL init_enclave:  3.120089 seconds
[Clocker] ECALL upload_query_data:  0.029946 seconds
[Clocker] Distribute central data:  8.888972 seconds
[Clocker] ECALL private_contact_trace:  0.187760 seconds
[Clocker] ECALL get_result:  0.215290 seconds


RUST_BACKTRACE=1 bin/app 1200000 tools/trajectory_hash/data_expanded/tokyo 49999 tools/trajectory_hash/data_expanded/tokyo/server-14000-24-22.csv
[Clocker] Read Query Data:  0.164364 seconds
[Clocker] Distribute central data:  8.882627 seconds
[Clocker] ECALL private_contact_trace:  0.189648 seconds
[Clocker] ECALL init_enclave:  3.124965 seconds
[Clocker] ECALL get_result:  0.218749 seconds
[Clocker] Read Central Data:  21.572727 seconds
[Clocker] ECALL upload_query_data:  0.030239 seconds


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
