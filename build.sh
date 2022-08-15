#!/bin/bash

if [[ $1 eq "run"]]; then
    echo Running docker image as ghems-live
    docker run -it --rm --name ghems-live ghems
else
    echo Building docker image (ghems)
    docker build -t ghems .
fi
