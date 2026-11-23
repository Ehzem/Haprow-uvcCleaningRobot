# Record CPU usage during build, Gazebo, RViz and keyboard tests

The Linux recorder uses Python's standard library and read-only `/proc` and
cgroup-v2 counters. It does not need ROS sourcing or extra monitoring packages.
Start it **before** the Terminal 1 build/launch commands, then run the simulation
and keyboard controller normally. Stop with Ctrl+C after your experiment.

## Existing running Docker container (no image rebuild required)

Run these from a **third host terminal**, leaving the simulation and keyboard
terminals alone. First identify the container in which both ROS terminals run:

```bash
cd ~/addison_ws
sudo docker ps
```

Replace `CONTAINER_ID` below with that container's ID. Copy the new recorder into
that container and start it with an interactive terminal so Ctrl+C stops it:

```bash
sudo docker cp docker/record_cpu.py CONTAINER_ID:/tmp/record_cpu.py
sudo docker exec -it CONTAINER_ID python3 /tmp/record_cpu.py \
  --interval 0.5 --output /home/itadmin/addison_ws/cpu_recordings/theatre_01
```

Use a new output name for each run; existing results are never overwritten.
The recorder logs every visible process, so processes that start later during
colcon builds or ROS launches are included if observed during sampling. You
can run this before launching ROS and leave it active during keyboard testing.

After pressing Ctrl+C in the recorder terminal, copy the results onto the host
**before exiting/removing a temporary simulation container**:

```bash
mkdir -p cpu_recordings
sudo docker cp CONTAINER_ID:/home/itadmin/addison_ws/cpu_recordings/. ./cpu_recordings/
```

A development-volume workspace keeps those files after its container is removed;
a disposable `run --rm` container without the development overlay does not.
The copy command works for either case while the container still exists.

## Updated checkout or newly rebuilt image

If `docker/record_cpu.py` is already in your container workspace, run this in a
third terminal **inside the same container**:

```bash
cd /home/itadmin/addison_ws
python3 docker/record_cpu.py --interval 0.5
```

It creates a timestamped subdirectory under `cpu_recordings/` and prints its
location. Future rebuilt images also install the equivalent shortcut:

```bash
workspace-record-cpu --interval 0.5
```

To stop automatically after two minutes:

```bash
python3 docker/record_cpu.py --interval 0.5 --duration 120
```

On a native Linux host, the same Python command works directly. Process records
then cover visible host processes; its cgroup metrics describe the recorder's
host service/cgroup, **not automatically a Docker container**. Run inside Docker
to measure that container's cgroup CPU usage.

## Files and percentage meanings

| File | Content |
| --- | --- |
| `system.csv` | Host-wide total and each logical core: active CPU, user/kernel, idle, I/O wait, interrupts and steal percentages |
| `processes.csv` | Visible PID, name, role, thread count, last scheduled CPU, user/kernel CPU percentages and CPU seconds observed in each interval |
| `cgroup.csv` | Recorder cgroup CPU usage, configured-capacity percentage and throttling deltas; normally the entire container when recorded inside Docker |
| `metadata.json` | Kernel, requested interval, tick resolution, host logical-CPU count, affinity, cgroup path and own quota |
| `summary.json` | Recorded duration, mean/peak sampled host usage, cgroup CPU seconds and top 25 processes by observed CPU time |

- `cpu_pct_one_core`: **100% means one logical CPU fully occupied**. A
  multithreaded Gazebo process can exceed 100%; 250% corresponds to 2.5 cores.
- `cpu_pct_host_capacity`: process utilization divided by the online host
  logical-CPU count. This uses the whole host as the denominator, not Docker's
  quota.
- `cpu_pct_configured_capacity`: cgroup utilization normalized to the recorder's
  affinity and the group's own `cpu.max` quota, if present. Restrictions imposed
  by parent cgroups are not inferred. Missing/unreadable cgroup-v2 data stays
  blank rather than being reported as zero.
- `system.csv` is **host-wide even inside Docker**. Other host applications
  affect it. Use process/cgroup files to distinguish container work.
- Per-process CPU includes all threads in that process. `last_cpu` is the last
  scheduled core ID, not that process's full per-core distribution. The `role`
  field is a heuristic; use PID and process name when analyzing uncertain tags.

All timestamps are explicitly UTC. Sampling uses a monotonic clock and the
actual measured interval. No process command-line arguments or credentials are
written. The recorder itself contributes some CPU overhead and is labelled
`recorder` in the process CSV.

This is kernel-counter sampling, **not a cycle-by-cycle trace**. Short-lived
processes can start and exit between samples, so their per-PID CPU can be missed;
host/container aggregate counters still account for their execution. A process
exiting between samples can also have an unobserved final CPU tail. Lower
`--interval` for finer detail, with increased recording overhead. Peaks are
sample-interval averages, not instantaneous spikes. GPU utilization is not
recorded by this CPU tool.

Record a short idle baseline, then build, launch Gazebo/RViz, and test motion
while the same recorder remains running. Analyze CSVs in a spreadsheet or with
Python. Recordings are excluded from Git by `/cpu_recordings/`.

## Validation

Run the accounting/CLI regression tests from the workspace root:

```bash
python3 -m unittest discover -s docker/tests -p 'test_record_cpu.py' -v
```

Tests cover CPU math, PID reuse, process names with parentheses, cgroup scope,
saved CSV output, refusal to overwrite recordings, and Ctrl+C finalization. A
Docker workload test with `--cpus 0.5` also passed, detecting the half-core quota
and recording both process and container CPU time.

Counter definitions: [Linux /proc documentation](https://www.kernel.org/doc/html/latest/filesystems/proc.html)
and [cgroup v2 CPU accounting](https://www.kernel.org/doc/html/latest/admin-guide/cgroup-v2.html).
