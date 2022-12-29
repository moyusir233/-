#!/bin/bash
./build_champsim.sh perceptron no no lru 4
./build_champsim.sh perceptron no bop lru 4
./build_champsim.sh perceptron no daampm lru 4
./build_champsim.sh perceptron no spp_dev_orig lru 4
./build_champsim.sh perceptron no spp_dev lru 4
./build_champsim.sh perceptron no spp_dev_zhu lru 4

./build_champsim.sh perceptron no no lru 1
./build_champsim.sh perceptron no bop lru 1
./build_champsim.sh perceptron no daampm lru 1
./build_champsim.sh perceptron no spp_dev_orig lru 1
./build_champsim.sh perceptron no spp_dev lru 1
./build_champsim.sh perceptron no spp_dev_zhu lru 1