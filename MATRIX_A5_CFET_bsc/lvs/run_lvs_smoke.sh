#!/usr/bin/env bash
# run_lvs_smoke.sh — run LVS on the 6 strict-BSC VBPR smoke cells,
# print PASS/FAIL per cell. Designed to be re-runnable.
set -u

SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)
PDK_DIR=$(cd "$SCRIPT_DIR/.." && pwd)

CELLS=$(grep -v '^#' "$PDK_DIR/07_pex/script/celllist_smoke")

echo "==== LVS smoke (6 cells) ===="
fail=0
for cell in $CELLS; do
    echo "---- $cell ----"
    bash "$SCRIPT_DIR/run_lvs.sh" "$cell" > "/tmp/${cell}.lvs.stdout" 2>&1
    rc=$?
    res=$(head -1 "$SCRIPT_DIR/results/${cell}.icv.lvs/${cell}.RESULTS" 2>/dev/null || echo "(no RESULTS)")
    printf '  %-44s %s  rc=%d\n' "$cell" "$res" "$rc"
    case "$res" in
        *PASS*) : ;;
        *) fail=$((fail+1)) ;;
    esac
done

echo
if [ $fail -eq 0 ]; then
    echo "PASS — all 6 smoke cells LVS-clean"
    exit 0
else
    echo "FAIL — $fail smoke cell(s) did not pass LVS; check /tmp/<cell>.lvs.stdout + results/<cell>.icv.lvs/icv.log"
    exit 1
fi
