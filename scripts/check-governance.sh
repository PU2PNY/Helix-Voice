#!/usr/bin/env bash
set -euo pipefail

required_files=(
  PROJECT_START_HERE.md
  PROJECT_MASTER_SPEC.md
  PROJECT_RELEASE_STATUS.md
  PROJECT_TEST_MATRIX.md
  PROJECT_KNOWN_GOOD.md
  PROJECT_PATENT_REGISTER.md
  PROJECT_CHANGELOG.md
  CLEAN_ROOM.md
  PATENT_GUARDRAILS.md
  AGENTS.md
  .github/copilot-instructions.md
  docs/PATENT_REVIEW_TEMPLATE.md
  docs/patent-reviews/hvc-v0.md
)

for file in "${required_files[@]}"; do
  if [[ ! -s "$file" ]]; then
    echo "governance error: required file missing or empty: $file" >&2
    exit 1
  fi
done

grep -Fq "PATENT_GUARDRAILS.md" PROJECT_START_HERE.md
grep -Fq "PROJECT_PATENT_REGISTER.md" PROJECT_START_HERE.md
grep -Fq "PROJECT_KNOWN_GOOD.md" PROJECT_START_HERE.md
grep -Fq "PATENT_GUARDRAILS.md" AGENTS.md
grep -Fq "PROJECT_PATENT_REGISTER.md" AGENTS.md
grep -Fq "PATENT_GUARDRAILS.md" .github/copilot-instructions.md
grep -Fq "PROJECT_PATENT_REGISTER.md" .github/copilot-instructions.md

grep -Fq "HVC v0" PROJECT_PATENT_REGISTER.md
grep -Fq "UNCERTAIN" docs/patent-reviews/hvc-v0.md
grep -Fq "produção bloqueada" PROJECT_PATENT_REGISTER.md ||   grep -Fq "BLOQUEADA" PROJECT_PATENT_REGISTER.md

if grep -Eiq 'HVC v0.*(production ready|production-ready|patent[- ]free|freedom to operate: clear)' README.md PROJECT_RELEASE_STATUS.md; then
  echo "governance error: HVC v0 must not be promoted as patent/FTO-cleared production codec" >&2
  exit 1
fi

echo "Governance/patent gates: PASS"
