#!/usr/bin/env fish

argparse 's-safe' 'd-dirty' 'r-no-reset'  -- $argv

set src (readlink -f (status dirname))

if ! set -q STAGING
    posix-source $src/.env
end

if ! set -q STAGING
    echo "$STAGING is not set."
    exit 1
end

mkdir -p $STAGING
cd $STAGING
and set -q _flag_no_reset || git reset --hard
and git pull --rebase
and cp -r $src/* $STAGING/
or exit

not set -q _flag_dirty || exit 0
git add --all
and git commit
or exit
not set -q _flag_safe || exit 0
git push || exit
