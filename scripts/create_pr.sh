#!/bin/bash

# Script to automate PR creation from branch to branch
# Usage: ./scripts/create_pr.sh <branch_name> <commit_message> <issue_number>

set -e

BRANCH_NAME=$1
COMMIT_MESSAGE=$2
ISSUE_NUMBER=$3

if [ -z "$BRANCH_NAME" ] || [ -z "$COMMIT_MESSAGE" ] || [ -z "$ISSUE_NUMBER" ]; then
    echo "Usage: $0 <branch_name> <commit_message> <issue_number>"
    exit 1
fi

echo "Creating branch: $BRANCH_NAME"
git checkout -b "$BRANCH_NAME"

echo "Adding changes..."
git add .

echo "Committing changes with message: $COMMIT_MESSAGE"
git commit -m "$COMMIT_MESSAGE" -m "Fixes #$ISSUE_NUMBER"

echo "Pushing branch to origin..."
git push origin "$BRANCH_NAME"

if command -v gh &> /dev/null; then
    echo "Creating PR using GitHub CLI..."
    gh pr create --title "$COMMIT_MESSAGE" --body "Fixes #$ISSUE_NUMBER" --base main --head "$BRANCH_NAME"
else
    echo "GitHub CLI (gh) not found. Please create the PR manually."
    echo "Branch pushed: $BRANCH_NAME"
fi

echo "Returning to main branch..."
git checkout main
