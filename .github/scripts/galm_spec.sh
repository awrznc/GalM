#!/bin/bash -eu

SCRIPT_DIR=$(cd $(dirname $0); pwd)
GALM_DIR=${SCRIPT_DIR}/../../target/release/examples

echo 'galm "王様レストラン" --dictionary "皇様レストラン,玉様レストラン,大様レストラン"'
result=`${GALM_DIR}/galm "王様レストラン" --dictionary "皇様レストラン,玉様レストラン,大様レストラン"`

if [ "${result}" = "玉様レストラン" ]; then
    echo -e "\e[32m    => '${result}' ok! \e[m"
else
    echo -e "\e[31m    => '${result}' error. \e[m"
    exit 1
fi

echo 'galm --version'
result=`${GALM_DIR}/galm --version`

if [[ "${result}" =~ ^(galm version ([0-9]+\.).{3})$ ]]; then
    echo -e "\e[32m    => '${result}' ok! \e[m"
else
    echo -e "\e[31m    => '${result}' error. \e[m"
    exit 1
fi
