#!/bin/bash
tmux new-window -d -n REPL-CCL "ccl -l ~/.vim/bundle/slimv/slime/start-swank.lisp"
