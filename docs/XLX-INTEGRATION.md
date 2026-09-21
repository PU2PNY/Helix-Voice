# XLX / XLXD Integration Plan

## Objective

Integrate HVE without destabilizing the existing reflector.

## Initial strategy

Use a separate Helix process and preserve the existing XLXD process boundary.

The first implementation should operate in **observe-only/laboratory mode** before it is allowed to transform production audio.

## Rules

- Do not replace a working transcoder without backup.
- Do not remove an existing hardware/licensed backend.
- Bind laboratory services to loopback unless remote access is explicitly required.
- Keep old service units available for rollback.
- Record before/after audio samples only when legally redistributable.
- Measure stream latency, timeout rate, CPU and RSS.
- Any proprietary codec capability must remain behind an external authorized/licensed backend.

## Acceptance before production

- 24-hour soak without memory growth beyond the agreed budget;
- no dropped XLXD service;
- deterministic rollback tested;
- no change to modules/protocol routing outside the test path;
- equal-or-better measured level consistency;
- no clipping regression;
- no material latency regression;
- failure of Helix does not corrupt XLXD configuration.
