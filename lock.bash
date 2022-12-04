#!/bin/bash

encrypt () {
	local working_dir=$1
	local recipient=$2
	if [[ -f "${working_dir}input.txt.gpg" ]]; then
		# Nothing to do here
		return
	fi

	if [[ -f "${working_dir}input.txt" ]]; then
		gpg --output ${working_dir}input.txt.gpg --encrypt --recipient ${recipient} ${working_dir}input.txt
	fi
}


for target in $(fd 'd[0-2][0-9]'); do
	encrypt ${target} $1
done
