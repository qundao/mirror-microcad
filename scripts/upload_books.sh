#!/bin/sh

# Check for flags
DRY_RUN=""
SHOW_ONLY=""
for arg in "$@"; do
    case "$arg" in
        --dry-run|-d)
            DRY_RUN="--dry-run"
            echo "DRY-RUN MODE - No files will be uploaded"
            echo ""
            ;;
        --show-command|-s)
            SHOW_ONLY="true"
            ;;
    esac
done

# Get script directory and project root
SCRIPT_DIR=$(dirname "$0")
PROJECT_ROOT=$(cd "$SCRIPT_DIR/.." && pwd)
BOOKS_DIR="$PROJECT_ROOT/books"

# Load .env file
if [ -f "$SCRIPT_DIR/.env" ]; then
    export $(grep -v '^#' "$SCRIPT_DIR/.env" | xargs)
else
    echo "Error: .env file not found!"
    exit 1
fi

# Check if all variables are set
if [ -z "$FTP_HOST" ] || [ -z "$FTP_USER" ] || [ -z "$FTP_PW" ]; then
    echo "Error: SFTP credentials incomplete in .env"
    exit 1
fi

# Set default values
FTP_PORT="${FTP_PORT:-21}"
FTP_REMOTE_PATH="${FTP_REMOTE_PATH:-.}"


# Build lftp commands dynamically
echo "Generate the command..."
LFTP_COMMANDS="set ftp:ssl-force true;"
# LFTP_COMMANDS=""
for book_dir in "$BOOKS_DIR"/*/book/; do
    if [ -d "$book_dir" ]; then
        book_name=$(echo "$book_dir" | awk -F'/' '{print $(NF-2)}')
        LFTP_COMMANDS="$LFTP_COMMANDS mirror -R --delete --verbose $DRY_RUN $book_dir $FTP_REMOTE_PATH/$book_name/;"
    fi
done
LFTP_COMMANDS="$LFTP_COMMANDS quit"

# Only show the command
if [ -n "$SHOW_ONLY" ]; then
    echo "LFTP command:"
    echo "lftp -e \"$LFTP_COMMANDS\" ftp://\$FTP_USER:\$FTP_PW@$FTP_HOST:$FTP_PORT"
    exit 0
fi

# Upload via lftp
echo "Starting upload..."
lftp -e "$LFTP_COMMANDS" ftp://$FTP_USER:$FTP_PW@$FTP_HOST:$FTP_PORT

if [ $? -eq 0 ]; then
    if [ -n "$DRY_RUN" ]; then
        echo ""
        echo "Dry-run complete! Run without --dry-run to upload."
    else
        echo ""
        echo "Upload successful!"
    fi
else
        echo ""
    echo "Upload failed!"
    exit 1
fi