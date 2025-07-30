#!/bin/bash

function start_jzfs_git() {
  echo "Starting jzfs_git..."
  sh -c /explore/jzfs & > /dev/null 2>&1
}

function start_nginx() {
  echo "Starting nginx..."
  nginx -g "daemon off;"
}

start_jzfs_git();
start_nginx();