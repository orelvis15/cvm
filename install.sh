#!/bin/bash

repo="https://api.github.com/repos/orelvis15/cvm/releases/latest"
env="https://raw.githubusercontent.com/orelvis15/cvm/master/env"

resource="uname"

os=$(uname -s)
arc=$(uname -m)

if [[ $os == "Linux" && $arc == "x86_64" ]]; then resource="cvm-x86_64"; fi

if [[ $os == "Linux" && $arc == "aarch64" ]]; then resource="cvm-aarch64"; fi

if [[ $resource == "uname" ]]; then
  echo "this architecture is not yet supported"
  exit
fi

#get the url of the artifact
artifact=$(curl -s $repo | awk -F\" '/browser_download_url.*.'$resource'.tar.gz/{print $(NF-1)}')

mkdir -p "$HOME"/.cvm

curl -L "$env" -o "$HOME"/.cvm/env
sed -i "s|home|$HOME|g" "$HOME"/.cvm/env

curl -L "$artifact" -o "$HOME"/.cvm/cvm.tar.gz

tar -xzf "$HOME"/.cvm/cvm.tar.gz -C "$HOME"/.cvm/
rm "$HOME"/.cvm/cvm.tar.gz
chmod 700 "$HOME"/.cvm/cvm
