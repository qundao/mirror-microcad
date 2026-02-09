#!/bin/bash
rm test-errors.log
find -name "*.log" | grep -E "doc/|book/" | while read -r file; do
    if grep -q "error" "$file" || grep -q "warning" "$file"; then
        echo >> test-errors.log
        echo "========= $file =========" >> test-errors.log
        echo >> test-errors.log
        cat "$file" >> test-errors.log
    fi
done
