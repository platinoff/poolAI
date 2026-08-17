#!/usr/bin/env bash
# PH-S201: Cursor postToolUse — remind VDT docs sync after successful git push (PH-S* commit).
set -euo pipefail

json_escape() {
  local s=$1
  s=${s//\\/\\\\}
  s=${s//\"/\\\"}
  s=${s//$'\n'/\\n}
  s=${s//$'\r'/}
  printf '%s' "$s"
}

emit_empty() {
  printf '{}\n'
}

read_stdin() {
  if [[ -n "${POOLAI_HOOK_TEST_INPUT:-}" ]]; then
    printf '%s' "$POOLAI_HOOK_TEST_INPUT"
    return
  fi
  cat
}

is_git_push_success() {
  local payload=$1
  if ! grep -qiE 'git[[:space:]]+push' <<<"$payload"; then
    return 1
  fi
  if grep -qE '"exitCode"[[:space:]]*:[[:space:]]*0' <<<"$payload"; then
    return 0
  fi
  if grep -qE '"exit_code"[[:space:]]*:[[:space:]]*0' <<<"$payload"; then
    return 0
  fi
  # tool_output may be escaped; loose match when push ran and no explicit failure
  if grep -qiE 'git[[:space:]]+push' <<<"$payload" && ! grep -qE '"exitCode"[[:space:]]*:[[:space:]]*[1-9]' <<<"$payload"; then
    return 0
  fi
  return 1
}

detect_sprint_id() {
  local subject=$1
  grep -oE 'PH-S[0-9]+' <<<"$subject" | head -1
}

read_next_sprint_hint() {
  local f="docs/development/NEXT_SESSION_PROMPT.md"
  if [[ ! -f "$f" ]]; then
    return
  fi
  grep -m1 'наступний' "$f" 2>/dev/null | grep -oE 'PH-S[0-9]+' | head -1 || true
}

run_self_test() {
  export POOLAI_HOOK_TEST_INPUT='{"tool_name":"Shell","tool_input":{"command":"git push origin main"},"tool_output":"{\"exitCode\":0,\"stdout\":\"main -> main\"}"}'
  export POOLAI_HOOK_TEST_SUBJECT='feat(vision): PH-S201 post-push hook'
  local out
  out="$(main_logic)"
  if ! grep -q 'additional_context' <<<"$out"; then
    echo "self-test failed: expected additional_context" >&2
    echo "$out" >&2
    return 1
  fi
  if ! grep -q 'PH-S201' <<<"$out"; then
    echo "self-test failed: expected PH-S201 in context" >&2
    return 1
  fi
  echo "post-push-ph-s-notify.sh self-test OK"
}

main_logic() {
  local input payload subject sprint next hint ctx escaped

  input="$(read_stdin)"
  payload="$input"

  if ! grep -qE '"tool_name"[[:space:]]*:[[:space:]]*"Shell"' <<<"$payload"; then
    emit_empty
    return 0
  fi

  if ! is_git_push_success "$payload"; then
    emit_empty
    return 0
  fi

  if [[ -n "${POOLAI_HOOK_TEST_SUBJECT:-}" ]]; then
    subject="$POOLAI_HOOK_TEST_SUBJECT"
  else
    subject="$(git log -1 --format=%s 2>/dev/null || true)"
  fi

  sprint="$(detect_sprint_id "$subject" || true)"
  if [[ -z "$sprint" ]]; then
    emit_empty
    return 0
  fi

  next="$(read_next_sprint_hint || true)"
  hint=""
  if [[ -n "$next" && "$next" != "$sprint" ]]; then
    hint="Next sprint (FM §5.12): ${next}."
  fi

  ctx="✅ git push succeeded (${sprint} in HEAD subject).

VDT docs-sync checklist (same session if not done):
- docs/catalog/FUNCTION_MANAGEMENT.md — mark ${sprint} ✅ in §5.12
- docs/development/HANDOFF_NEW_SESSION.md
- docs/development/NEXT_SESSION_PROMPT.md
- docs/vision/manifest.json, extensions.json, feed.json (cargo run --bin poolai-vision-sync)
- docs/vision/vision.svg footer; docs/vision/README.md

Canon: .cursor/commands/git-push.md · poolai-session-iteration.mdc
${hint}"

  escaped="$(json_escape "$ctx")"
  printf '{"additional_context":"%s"}\n' "$escaped"
}

if [[ "${1:-}" == "--self-test" ]]; then
  run_self_test
  exit $?
fi

main_logic
