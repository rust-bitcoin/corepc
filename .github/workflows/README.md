# corepc workflow notes

Because there are so many integration test jobs (one for each version
of Core supported) we cannot stay under the 20 job limit.

ref: https://docs.github.com/en/actions/learn-github-actions/usage-limits-billing-and-administration

The minimal/recent lock files are handled by CI (`rust.yml`).

## Jobs

Run from `rust.yml` unless stated otherwise. Total 12 jobs.

0.  `Prepare`
1.  `Stable - minimal`
2.  `Stable - recent`
3.  `Nightly - minimal`
4.  `Nightly - recent`
5.  `MSRV - minimal`
6.  `MSRV - recent`
7.  `Lint`
8.  `Docs`
9.  `Docsrs`
10. `Format`
11. `Verify`
12. `Codegen`

+16 jobs - 1 for each supported version of Core.

+1 job - integration tests run against the async client (v30 only).
