#!/usr/bin/env bash
set -euo pipefail

start_dir=$(pwd)
cd ~
if [[ -d ".nixt" ]];then
  rm -rf .nixt/
  echo "[-] Removing existing ~/.nixt directory"
fi
mkdir .nixt/
echo "[+] Creating a new ~/.nixt directory"
cd .nixt/
wget -q https://github.com/Wafelack/nixt/releases/download/0.1.0-alpha/nixt.tar.gz
echo "[+] Cloning release from repo"
tar -xf nixt.tar.gz
echo "[+] Extracting cloned archive"
echo 'export PATH="~/.nixt/:$PATH"' >> ~/.bashrc
echo 'export NIXT_PATH="~/.nixt/std"' >> ~/.bashrc
echo "[+] Added nixt to path"
cd $start_dir
echo "Successfully installed nixt !"
