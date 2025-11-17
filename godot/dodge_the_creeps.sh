#!/bin/sh
printf '\033c\033]0;%s\a' godot
base_path="$(dirname "$(realpath "$0")")"
"$base_path/dodge_the_creeps.x86_64" "$@"
