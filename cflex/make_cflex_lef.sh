#!/bin/sh
# ---------------------------------------------------------------------------
# make_cflex_lef.sh -- prepare the iso-height CFlex LEF set.
#
# MATRIX_A5_CFET_base, _uniform and _mismatch are three 4-track libraries at
# the SAME row height. Their tech LEFs are byte-identical apart from the SITE
# name, and each library's cells reference its own site:
#
#     MATRIX_A5_CFET_base      SITE CoreSite         0.039 x 0.072
#     MATRIX_A5_CFET_uniform   SITE CORESITE_20      0.039 x 0.072
#     MATRIX_A5_CFET_mismatch  SITE CORESITE_11_20   0.039 x 0.072
#
# Place-and-route builds rows from one site, so cells carrying the other two
# site names will not legalize. This script writes copies of the _uniform and
# _mismatch macro/filler LEFs with their SITE renamed to the common CoreSite,
# into cflex/lef/. Nothing in the five library directories is modified.
#
# Usage:  sh cflex/make_cflex_lef.sh
# ---------------------------------------------------------------------------
set -e

here=$(cd "$(dirname "$0")" && pwd)
root=$(dirname "$here")
out="$here/lef"
site=CoreSite

mkdir -p "$out"

# base already uses $site -- copy unchanged
cp "$root/MATRIX_A5_CFET_base/lef/gtA5_cfet_macros.lef"     "$out/"
cp "$root/MATRIX_A5_CFET_base/lef/gtA5_cfet_11_fillers.lef" "$out/"

# uniform + mismatch -- rename the cell SITE to $site
for rel in \
  MATRIX_A5_CFET_uniform/lef/gtA5_cfet_20_macros.lef \
  MATRIX_A5_CFET_uniform/lef/gtA5_cfet_20_fillers.lef \
  MATRIX_A5_CFET_mismatch/lef/gtA5_cfet_11_20_macros.lef \
  MATRIX_A5_CFET_mismatch/lef/gtA5_cfet_11_20_fillers.lef
do
  sed -E "s/^([[:space:]]*SITE[[:space:]]+)CORESITE_(20|11_20)([[:space:]]*;)/\1${site}\3/" \
      "$root/$rel" > "$out/$(basename "$rel")"
done

echo "CFlex LEF set written to $out:"
for f in "$out"/*.lef; do
  printf '  %-34s %3s MACRO  SITE %s\n' "$(basename "$f")" \
      "$(grep -c '^MACRO' "$f")" \
      "$(grep -m1 '^[[:space:]]*SITE' "$f" | awk '{print $2}')"
done
echo
echo "Load with the tech LEF from MATRIX_A5_CFET_base (it defines SITE $site):"
echo "  MATRIX_A5_CFET_base/lef/gtA5_cfet.tlef  +  the six LEFs above"
