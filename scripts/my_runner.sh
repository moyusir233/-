#!/bin/bash
trace1=/home/ubuntu/PPF_Paper_Source_Code-master/traces/603.bwaves_s-1080B.champsimtrace.xz
trace2=/home/ubuntu/PPF_Paper_Source_Code-master/traces/605.mcf_s-1152B.champsimtrace.xz
trace3=/home/ubuntu/PPF_Paper_Source_Code-master/traces/623.xalancbmk_s-10B.champsimtrace.xz
trace4=/home/ubuntu/PPF_Paper_Source_Code-master/traces/649.fotonik3d_s-10881B.champsimtrace.xz

../bin/perceptron-no-no-lru-4core -warmup_instructions 20000000 -simulation_instructions 100000000  -traces $trace1 $trace2 $trace3 $trace4 | tee tmp_result.txt