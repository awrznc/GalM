#!/bin/bash -eu

# Required Command:
#   grep, gsed, sort, uniq, awk, head, echo
#   git, cargo

# Required Env:
#   GITHUB_TOKEN, GITHUB_REF, GITHUB_ACTOR, GITHUB_REPOSITORY
#   EMAIL_FOR_AUTO_COMMIT, CRATES_IO_API_ACCESS_KEY, CARGO

# Get Version
echo -n "Get Version ..."
SCRIPT_DIR=$(cd $(dirname $0); pwd)
old_version=`grep -E "\"[0-9]+?\.[0-9]+?\.[0-9]+\"" -rho ${SCRIPT_DIR}/../../ --exclude-dir=target | \
    sort | \
    uniq -c | \
    awk '{print $2}' | \
    head -n 1`
build_no=`echo ${old_version} | sed -E "s/^.*(\"[0-9]+?\.[0-9]+?\.)([0-9]+)(\").*$/\2/"`
major_and_minor_version=`echo ${old_version} | sed -E "s/^.*(\"[0-9]+?\.[0-9]+?\.)([0-9]+)(\").*$/\1/"`
new_version=`echo ${major_and_minor_version}$(expr ${build_no} + 1)'"'`
message="Updated Build No ${old_version} -> ${new_version}."
echo -e "\e[32m finished! \e[m"

# Print Version
echo -e "\e[33m${message}\e[m"

# Replace Version
echo -n "Replace Version ..."
grep ${old_version} -rl ${SCRIPT_DIR}/../../ --exclude-dir=target | xargs sed -i "s/${old_version}/${new_version}/g"
echo -e "\e[32m finished! \e[m"

# GitHub
echo -n "Git Commit ..."
git config --global user.name ${GITHUB_ACTOR}
git config --global user.email ${EMAIL_FOR_AUTO_COMMIT}
git remote set-url origin https://${GITHUB_ACTOR}:${GITHUB_TOKEN}@github.com/${GITHUB_REPOSITORY}.git
git checkout ${GITHUB_REF}
git commit -am "${message}"
git push origin ${GITHUB_REF}
echo -e "\e[32m finished! \e[m"

# Publish
echo -n "Cargo Publish ..."
eval "${CARGO}" login ${CRATES_IO_API_ACCESS_KEY}
eval "${CARGO}" publish
echo -e "\e[32m finished! \e[m"
