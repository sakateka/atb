#!/bin/bash

while true; do
    count=$(($(($RANDOM % 9)) + 2))
    numbers=()
    for rn in $(seq 0 $((count - 1))); do
        numbers[rn]="$RANDOM "
    done
    data="$count\n${numbers[@]}"
    echo count=$count dataset_len=${#numbers[@]}
    echo data set: ${numbers[@]}
    echo run fast version
    fast_result=$(echo -e "$data" | ./max_pariwise_product)
    echo run naive version
    naive_result=$(echo -e "$data" | ./max_pariwise_product_naive)
    if ((fast_result > 0 && fast_result == naive_result)); then
        echo OK
    else
        echo "Wrong result expect=$naive_result, actual=$fast_result"
        exit 1
    fi
done
