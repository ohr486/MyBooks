#!/bin/bash

CH=ch24
EDITION='--edition 2021'
OPT='-O'

cd $CH && rustc $EDITION $OPT ./main.rs && ./main $@
