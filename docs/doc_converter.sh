#!/bin/bash
set -e
cd /home/hipno/Diwan/docs/ 
#dirpath=$(realpath $(find . -type d -name "*src*" -print -quit))
#echo $dirpath
mdbook build
echo "Documentation has been built."
