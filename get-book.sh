#!/bin/bash
BOOK_NAME=$1
git sparse-checkout set "books/$BOOK_NAME"
echo "Checked out $BOOK_NAME only."
