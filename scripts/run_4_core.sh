#!/bin/bash
traces=(623.xalancbmk_s-10B.champsimtrace.xz 649.fotonik3d_s-10881B.champsimtrace.xz 603.bwaves_s-1080B.champsimtrace.xz 605.mcf_s-1152B.champsimtrace.xz)

binaries=(perceptron-no-no-lru-4core perceptron-no-bop-lru-4core perceptron-no-daampm-lru-4core  perceptron-no-spp_dev-lru-4core perceptron-no-spp_dev_orig-lru-4core)

for binary in "${binaries[@]}"; do
    (./bin/"${binary}" -warmup_instructions 200000000 -simulation_instructions 1000000000  -traces  "${traces[@]}") &> ../my_result/4_core_"${binary}".log
done