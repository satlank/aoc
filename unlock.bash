#!/bin/bash

decrypt () {
	local working_dir=$1
	if [[ -f "${working_dir}input.txt.gpg" ]]; then
		gpg --output ${working_dir}input.txt --decrypt ${working_dir}input.txt.gpg
	fi
}


for target in $(fd 'd[0-2][0-9]'); do
	decrypt ${target}
done
