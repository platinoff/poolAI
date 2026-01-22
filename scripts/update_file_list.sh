#!/bin/bash
# Update file_list.csv with current project files
# Usage: bash scripts/update_file_list.sh

set -e

cd "$(dirname "$0")/.."

echo "📋 Updating file_list.csv..."

# Find all files, exclude target, .git, node_modules
find . -type f \
  -not -path './target/*' \
  -not -path './.git/*' \
  -not -path './node_modules/*' \
  -not -path './.cursor/cache/*' \
  | sort > file_list_new.csv

# Check if file exists and is different
if [ -f file_list.csv ]; then
  if cmp -s file_list.csv file_list_new.csv; then
    echo "✅ file_list.csv is up to date"
    rm file_list_new.csv
    exit 0
  else
    echo "📝 Changes detected, updating file_list.csv..."
    mv file_list_new.csv file_list.csv
    echo "✅ file_list.csv updated"
  fi
else
  mv file_list_new.csv file_list.csv
  echo "✅ file_list.csv created"
fi

# Show statistics
TOTAL_FILES=$(wc -l < file_list.csv)
echo "📊 Total files: $TOTAL_FILES"
