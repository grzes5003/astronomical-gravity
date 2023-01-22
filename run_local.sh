#!/bin/bash -l

log_dir=./logs

cargo build --release

if [ -z "$SCRIPT" ]; then
  TODAY=$(date +"%d_%H_%M")
  exec 3>&1 4>&2
  trap 'exec 2>&4 1>&3' 0 1 2 3
  exec 1>"$log_dir"/log_"$TODAY".log 2>&1
fi

prog=./target/release/AR_proj
input_file_prefix=./resources/

declare -a arr=("result_50.csv" "result_100.csv" "result_182.csv")

for file_name in "${arr[@]}"; do
  file_path="$input_file_prefix""$file_name"
  for ((iter = 10; iter > 0; iter--)); do
    mpiexec -np 1 "$prog" -f "$file_path"
    for ((threads = 2; threads <= 6; threads += 2)); do
      mpiexec -np "$threads" "$prog" -f "$file_path"
    done
  done
done
