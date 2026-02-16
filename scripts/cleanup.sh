#!/bin/bash

# Function to get all versions across all pages
get_all_versions() {
  local page=1
  local per_page=100
  local all_versions="[]"

  while true; do
    echo "Fetching page $page..." >&2

    response=$(gh api \
      -H "Accept: application/vnd.github+json" \
      -H "X-GitHub-Api-Version: 2022-11-28" \
      "/orgs/PLC-lang/packages/container/rusty/versions?per_page=$per_page&page=$page")

    # Check if response is empty array
    if [ "$(echo "$response" | jq '. | length')" -eq 0 ]; then
      break
    fi

    # Append to all_versions
    all_versions=$(echo "$all_versions" | jq ". + $response")

    page=$((page + 1))
  done

  echo "$all_versions"
}

echo "Fetching all package versions..."
all_versions=$(get_all_versions)

total_count=$(echo "$all_versions" | jq '. | length')
echo "Found $total_count total versions"

# List versions to keep (master, latest)
echo ""
echo "Versions to KEEP:"
echo "$all_versions" | \
  jq -r '.[] | select(.metadata.container.tags // [] | any(. == "master" or . == "latest")) | "\(.id)\t\(.metadata.container.tags | join(", "))"'

# List versions to delete
echo ""
echo "Versions to DELETE:"
to_delete=$(echo "$all_versions" | \
  jq -r '.[] | select(.metadata.container.tags | if . then any(. == "master" or . == "latest") | not else true end) | .id')

delete_count=$(echo "$to_delete" | wc -l)
echo "Will delete $delete_count versions"

# Confirm before deleting
echo ""
read -p "Do you want to proceed with deletion? (yes/no): " confirm

if [ "$confirm" != "yes" ]; then
  echo "Aborted."
  exit 0
fi

# Delete versions in parallel
echo "$to_delete" | xargs -P 10 -I {} sh -c '
  echo "Deleting version {}..."
  gh api --method DELETE \
    -H "Accept: application/vnd.github+json" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    /orgs/PLC-lang/packages/container/rusty/versions/{}
  sleep 0.1
'

echo ""
echo "Cleanup complete!"
