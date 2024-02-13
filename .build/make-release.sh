#!/bin/bash

unset -v new_version

while getopts v: flag;
do
    case $flag in
        v) new_version=$OPTARG ;;
        *) echo "usage: $0 [-v]" >&2
    esac
done

shift "$(( OPTIND - 1 ))"

if [ -z "${new_version}" ]; then
        echo "-v flag is required" >&2
        exit 1
fi

previous_version=$(awk -F "= " '/^version/ {print $2}' ./Cargo.toml)
sed -i '' -e "s/^version = ""$previous_version""/version = ""\"$new_version""\"/g" ./Cargo.toml

git add ./Cargo.toml
git commit -m "Bump up version to v${new_version}"
git push
git tag v"$new_version"
git push origin v"$new_version"