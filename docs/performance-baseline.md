# Resource comparison — deferred to final validation

Date: October 17, 2026. Status: **deferred to final validation (P31/P38) by owner decision**.

The owner explicitly chose to measure and compare the legacy and Rust installations at the end. An initial 30-minute baseline does not block P03 or development. Retain the installation facts below and the current legacy path so both configurations can be measured under comparable conditions before legacy removal. Minimum memory/CPU remains a Rust server design constraint; no numeric gain is claimed in advance.

## Results

| Required evidence | Result |
|---|---|
| Target | QNAP system status confirms TS-251+, Intel Celeron J1900, up to 1900 MHz, 4 cores/4 threads, 2 GB RAM |
| QTS | 5.2.10.3577 Build 20260731, read from system status |
| Kernel | Not collected |
| Container Station | UI application URL reports 3.1.2.1742 |
| Homebridge image | `homebridge/homebridge:latest@sha256:7ead5294ef0b49ae3eaa305f67b8086259460a14b7d78a83ba41bae0deda269b` |
| Runtime/resource settings | Running; `runc`; privileged mode disabled; CPU limit, memory limit and memory reservation displayed as Unlimited; restart policy Always |
| Network/storage | Bridge mode through Qnet and a virtual switch; distinct LAN address omitted. UI port-forwarding field displays `--`. Writable host bind mount to `/homebridge`; host path omitted |
| Versions and 23-element scope | Rechecked in the existing authenticated Homebridge UI; see [compatibility](compatibility.md). No process measurement follows from these observations |
| Warmup, sample interval, start/end and 30-minute window | Not run |
| Required process list and parent/child relationships | Not collected |
| Per-process RSS/PSS and CPU | Not measured |
| Host memory and CPU | One UI observation: CPU 14.5%, memory 56.5% / 1.03 GB of 1.83 GB usable; not a baseline, RSS or PSS |
| Container current/peak memory and cache | Overview displayed transient percentages, but exact metric/cgroup definition, byte counters, peak and cache were not collected; no baseline result |
| Rust/native/hybrid comparison or savings | Not measured; no gain claimed |

The owner supplied the existing authenticated QNAP session in Safari. System status, Resource Monitor and Container Station were consulted read-only on October 17, 2026. The UI exposed a single running Homebridge container. No shell, terminal, raw Inspect export, logs, private files, credentials or pairing data were requested. No services were started, stopped, restarted, reconfigured, or installed.

These UI observations establish installation facts only. The process screen did not establish a complete required-process tree or RSS/PSS definitions, and no 30-minute sample series was collected. The instantaneous host figures above cannot be attributed to Homebridge alone; container overview percentages cannot substitute for cgroup byte counters. Local Mac measurements and CI build statistics would not establish the production baseline either.

## Read-only comparison protocol for final validation

1. Record authorized source, date/time zone, QTS/kernel/Container Station, image digest, versions, topology, network mode and resource limits in a Git-excluded private report. Record device counts, required functions, gateway connection state, and existing service uptime. Do not collect secrets, environment values, process command lines, pairing data or raw configuration into public output.
2. Observe the unchanged installation after at least 10 minutes of warmup; record actual warmup and existing activity. Do not restart services or command equipment to create a scenario. Mark incidental household activity and connection changes in the private timeline.
3. Sample every 10 seconds for at least 30 minutes (at least 180 samples, with timestamps and an actual elapsed window of at least 30 minutes). Enumerate all required processes and children: Homebridge parent, UI, child bridges/plugins, Node helpers and any other legacy helper. The replacement server/HomeKit transport must remain one Rust process. Use read-only process counters. Record PID/start identity so restarts and missing samples are visible; omit command-line arguments. If a metric is unavailable, report that explicitly rather than substituting zero or elevating permissions.
4. Keep per-process RSS, PSS where available, and CPU separate from host memory/CPU and Docker cgroup current/peak/cache. Record units, metric source/cgroup version, CPU normalization, and sample interval. Compute CPU from interval deltas where appropriate; do not treat cumulative CPU time as instantaneous utilization. RSS sums double-count shared pages; prefer summed PSS for comparison. Container memory includes other allocations/cache and is not interchangeable with process PSS.
5. Retain raw samples privately. Publish only a reviewed technical summary: covered functions/process classes, window, sample count, missing metrics, median/p95/max memory and CPU, errors/restarts, and whether activity was comparable. Do not publish internal addresses, container/host names, raw PIDs or household identifiers.
6. Future Rust comparisons use identical hardware, active functions, device counts, connection conditions, warmup, window and metric definitions. Include all retained legacy processes/helpers in the total. The proposed architecture targets remain targets until measured; a smaller unsupported device set cannot establish savings.

This protocol is preparation, not evidence that the baseline ran. At final validation, use a source exposing process/cgroup counters and privately retain comparable legacy and Rust sample series covering these fields. Missing measurements are explicitly deferred, not a P03 prerequisite. Physical control, migrations, forced failures and production changes remain outside P03.
