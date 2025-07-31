#!/bin/bash

echo "Starting jzfs_git..."
sh -c /explore/jzfs & > /dev/null 2>&1
echo "Starting nginx..."
nginx -g "daemon off;"