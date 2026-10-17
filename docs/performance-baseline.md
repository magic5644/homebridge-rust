# P03 — Resource baseline

Date: October 17, 2026. Status: **not measured; installation access missing**.

## Results

| Required evidence | Result |
|---|---|
| Target | QNAP TS-251+, Celeron J1900, 2 GB reported; not reverified |
| QTS, kernel, Container Station, image/digest and resource limits | Not collected |
| Versions and 23-element scope | Historical UI observations only; see [compatibility](compatibility.md) |
| Warmup, sample interval, start/end and 30-minute window | Not run |
| Required process list and parent/child relationships | Not collected |
| Per-process RSS/PSS and CPU | Not measured |
| Host memory and CPU | Not measured |
| Container current/peak memory and cache | Not measured |
| Rust/native/hybrid comparison or savings | Not measured; no gain claimed |

No NAS shell or resource-observation session was supplied. Local Mac measurements and CI build statistics would not establish the production baseline. The Homebridge memory widget is not per-process consumption. No services were started, stopped, restarted, reconfigured, or installed to obtain measurements.

## Read-only collection protocol when access is supplied

1. Record authorized source, date/time zone, QTS/kernel/Container Station, image digest, versions, topology, network mode and resource limits in a Git-excluded private report. Record device counts, required functions, gateway connection state, and existing service uptime. Do not collect secrets, environment values, process command lines, pairing data or raw configuration into public output.
2. Observe the unchanged installation after at least 10 minutes of warmup; record actual warmup and existing activity. Do not restart services or command equipment to create a scenario. Mark incidental household activity and connection changes in the private timeline.
3. Sample every 10 seconds for at least 30 minutes (at least 180 samples, with timestamps and an actual elapsed window of at least 30 minutes). Enumerate all required processes and children: Homebridge parent, UI, child bridges/plugins, Node helpers, and any required sidecar. Use read-only process counters. Record PID/start identity so restarts and missing samples are visible; omit command-line arguments. If a metric is unavailable, report that explicitly rather than substituting zero or elevating permissions.
4. Keep per-process RSS, PSS where available, and CPU separate from host memory/CPU and Docker cgroup current/peak/cache. Record units, metric source/cgroup version, CPU normalization, and sample interval. Compute CPU from interval deltas where appropriate; do not treat cumulative CPU time as instantaneous utilization. RSS sums double-count shared pages; prefer summed PSS for comparison. Container memory includes other allocations/cache and is not interchangeable with process PSS.
5. Retain raw samples privately. Publish only a reviewed technical summary: covered functions/process classes, window, sample count, missing metrics, median/p95/max memory and CPU, errors/restarts, and whether activity was comparable. Do not publish internal addresses, container/host names, raw PIDs or household identifiers.
6. Future Rust comparisons use identical hardware, active functions, device counts, connection conditions, warmup, window and metric definitions. Include retained legacy processes and every sidecar in the total. The proposed architecture targets remain targets until measured; a smaller unsupported device set cannot establish savings.

This protocol is preparation, not evidence that the baseline ran. Resume only with a supplied authorized read-only session or a privately collected report covering these fields. Physical control, migrations, forced failures and production changes remain outside P03.
