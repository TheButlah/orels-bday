#!/usr/bin/env bash

set -Eeuo pipefail

main() {
	SOURCE_URL="https://github.com/TheButlah/orels-bday"
	BINARY_URL="${SOURCE_URL}/releases/latest/download"

	echo "HAPPY BIRTHDAY ORBLS!!"
	echo "I hope you enjoy my birthday card :3"
	echo "its a little cute cli and definitely not a virus uwu"
	echo "hit enter to play, or build it yourself from ${SOURCE_URL}"
	read

	if [[ "$OSTYPE" == "linux-gnu"* ]]; then
		if [[ "$(uname -m)" == "aarch64" ]]; then
			TARGET="linux-aarch64" 
		else 
			TARGET="linux-x86_64" 
		fi
	elif [[ "$OSTYPE" == "darwin"* ]]; then
		TARGET="macos-aarch64"
	else
		echo "unsupported environment"
		exit 1
	fi
	TARGET="orels-${TARGET}"

	tmp_dir="$(mktemp -d)"
	pushd ${tmp_dir}
	curl -fsSL "${BINARY_URL}/${TARGET}" -O
	chmod +x "./${TARGET}"
	 "./${TARGET}"
	rm -rf ${tmp_dir}
}

main
