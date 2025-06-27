#!/usr/bin/bash

exe_name='puzzle-AOC-2024-1'


rustc -o ${exe_name} ${exe_name}.rs &&

mv ${exe_name} ~/exes/ &&

chmod +x ~/exes/${exe_name} &&

~/exes/${exe_name}


echo " "
echo "compilation completed with exit code $(echo $?)"








