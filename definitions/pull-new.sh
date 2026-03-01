curl -O https://raw.githubusercontent.com/github-linguist/linguist/refs/heads/main/lib/linguist/languages.yml > languages.yml
curl https://raw.githubusercontent.com/github-linguist/linguist/refs/heads/main/lib/linguist/heuristics.yml > heuristics_original.yml
curl -O https://raw.githubusercontent.com/github-linguist/linguist/refs/heads/main/lib/linguist/vendor.yml > vendor.yml

# Patch the upstream heuristics to rewrite Ruby-specific regex features unsupported
# by fancy-regex. Currently rewrites the Adblock Filter List pattern to inline the
# \g<version> subroutine call. heuristics_original.yml is kept for reference.
patch -o heuristics.yml heuristics_original.yml heuristics.patch
