#!/bin/bash

if [[ $(id -u) == 0 ]] ; then echo "Please don't run as root" ; exit 1 ; fi

repo="https://api.github.com/repos/orelvis15/cvm/releases/latest"
env="https://raw.githubusercontent.com/orelvis15/cvm/master/env"

resource="uname"
debug=""

if [[ $1 == "debug" ]]; then debug="-debug"; fi

os=$(uname -s)
arc=$(uname -m)

if [[ $os == "Linux" && $arc == "x86_64" ]]; then resource="cvm-x86_64"$debug; fi
if [[ $os == "Linux" && $arc == "aarch64" ]]; then resource="cvm-aarch64"$debug; fi

if [[ $resource == "uname" ]]; then
  echo "this architecture is not yet supported"
  exit
fi

#get the url of the artifact
artifact=$(curl -s $repo | awk -F\" '/browser_download_url.*.'$resource'.tar.gz/{print $(NF-1)}')

mkdir -p "$HOME"/.cvm
mkdir -p "$HOME"/.cvm/tmp

curl -L "$env" -o "$HOME"/.cvm/env
sed -i "s|home|$HOME|g" "$HOME"/.cvm/env

curl -L "$artifact" -o "$HOME"/.cvm/cvm.tar.gz

tar -xzf "$HOME"/.cvm/cvm.tar.gz -C "$HOME"/.cvm/
rm "$HOME"/.cvm/cvm.tar.gz
chmod 700 "$HOME"/.cvm/cvm

set_shell() {
  case $SHELL in
  */zsh)
    PROFILE_FILE="$HOME/.zshrc"
    MY_SHELL="zsh"
    ;;
  */bash)
    PROFILE_FILE="$HOME/.bashrc"
    MY_SHELL="bash"
    ;;
  */sh)
    if [ -n "${BASH}" ]; then
      PROFILE_FILE="$HOME/.bashrc"
      MY_SHELL="bash"
    elif [ -n "${ZSH_VERSION}" ]; then
      PROFILE_FILE="$HOME/.zshrc"
      MY_SHELL="zsh"
    else
      return
    fi
    ;;
  esac
}

adjust_bashrc() {
  case $MY_SHELL in
  bash | zsh)
    sed -i -e '/# cvm-env$/ s/^#*/#/' "$(posix_realpath "${PROFILE_FILE}")"
    printf "\n%s" "[ -f \"${HOME}/.cvm/env\" ] && source \"${HOME}/.cvm/env\" # cvm-env" >>"${PROFILE_FILE}"
    ;;
  esac
}

# @FUNCTION: posix_realpath
# @USAGE: <file>
# @DESCRIPTION:
# Portably gets the realpath and prints it to stdout.
# This was initially inspired by
#   https://gist.github.com/tvlooy/cbfbdb111a4ebad8b93e
#   and
#   https://stackoverflow.com/a/246128
#
# If the file does not exist, just prints it appended to the current directory.
# @STDOUT: realpath of the given file
posix_realpath() {
  [ -z "$1" ] && die "Internal error: no argument given to posix_realpath"
  current_loop=0
  max_loops=50
  mysource=$1
  # readlink and '[ -h $path ]' behave different wrt '/sbin/' and '/sbin', so we strip it
  mysource=${mysource%/}
  [ -z "${mysource}" ] && mysource=$1

  while [ -h "${mysource}" ]; do
    current_loop=$((current_loop + 1))
    mydir="$(cd -P "$(dirname "${mysource}")" >/dev/null 2>&1 && pwd)"
    mysource="$(readlink "${mysource}")"
    [ "${mysource%"${mysource#?}"}"x != '/x' ] && mysource="${mydir%/}/${mysource}"

    if [ ${current_loop} -gt ${max_loops} ]; then
      (echo >&2 "${1}: Too many levels of symbolic links")
      echo "$1"
      return
    fi
  done
  mydir="$(cd -P "$(dirname "${mysource}")" >/dev/null 2>&1 && pwd)"

  # TODO: better distinguish between "does not exist" and "permission denied"
  if [ -z "${mydir}" ]; then
    (echo >&2 "${1}: Permission denied")
    echo "$(pwd)/$1"
  else
    echo "${mydir%/}/$(basename "${mysource}")"
  fi

  unset current_loop max_loops mysource mydir
}

set_shell
adjust_bashrc