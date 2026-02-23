cd books
find -name \*.log | xargs grep TODO | sed -E 's|^.*\.test/([^.]+)\.log:TODO(.*)| \1\2|'