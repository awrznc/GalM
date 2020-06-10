#!/bin/bash -eu

SCRIPT_DIR=$(cd $(dirname $0); pwd)
GALM_DIR=${SCRIPT_DIR}/../../target/release/examples

echo 'galm --version'
result=`${GALM_DIR}/galm --version`

if [[ "${result}" =~ ^(galm version [0-9]+?\.[0-9]+?\.[0-9]+)$ ]]; then
    echo -e "\e[32m    => '${result}' ok! \e[m"
else
    echo -e "\e[31m    => '${result}' error. \e[m"
    exit 1
fi


echo 'galm "王様" --file ${SCRIPT_DIR}/data.txt'
result=`${GALM_DIR}/galm "王様" --file ${SCRIPT_DIR}/data.txt`
expect=`echo -e "玉様\n大様\n皇様"`

if [ "${result}" = "${expect}" ]; then
    echo -e "\e[32m    => '${result}' ok! \e[m"
else
    echo -e "\e[31m    => '${result}' error. \e[m"
    exit 1
fi

echo 'echo -e "皇様\n玉様\n大様" | ${GALM_DIR}/galm "王様"'
result=`echo -e "皇様\n玉様\n大様" | ${GALM_DIR}/galm "王様"`
expect=`echo -e "玉様\n大様\n皇様"`

if [ "${result}" = "${expect}" ]; then
    echo -e "\e[32m    => '${result}' ok! \e[m"
else
    echo -e "\e[31m    => '${result}' error. \e[m"
    exit 1
fi
