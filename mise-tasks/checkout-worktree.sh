#!/bin/bash

# args
# $1 : BRANCH_NAME = ブランチ名
git worktree add -B $1 ./.git/worktree/$1