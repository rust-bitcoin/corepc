# OpenRPC Bitcoin Core Specs

To generate OpenRPC specs for a new Bitcoin Core version, use LLM to backport PR [34683](https://github.com/bitcoin/bitcoin/pull/34683) into newer version, then run command `bitcoin-cli getopenrpcinfo` to generate the spec.

For older versions,

Bitcoin Core Version 31.0: https://github.com/satsfy/bitcoin/tree/31.x-openrpc-backport
Bitcoin Core Version 30.2: https://github.com/satsfy/bitcoin/tree/30.x-openrpc-backport
Bitcoin Core Version 29.2: https://github.com/satsfy/bitcoin/tree/29.x-openrpc-backport
Bitcoin Core Version 28.2: https://github.com/satsfy/bitcoin/tree/28.x-openrpc-backport
Bitcoin Core Version 27.2: https://github.com/satsfy/bitcoin/tree/27.x-openrpc-backport
Bitcoin Core Version 26.2: https://github.com/satsfy/bitcoin/tree/26.x-openrpc-backport
Bitcoin Core Version 25.2: https://github.com/satsfy/bitcoin/tree/25.x-openrpc-backport
Bitcoin Core Version 24.2: https://github.com/satsfy/bitcoin/tree/24.x-openrpc-backport
Bitcoin Core Version 23.2: https://github.com/satsfy/bitcoin/tree/23.x-openrpc-backport
Bitcoin Core Version 22.2: https://github.com/satsfy/bitcoin/tree/22.x-openrpc-backport
Bitcoin Core Version 0.21.2: https://github.com/satsfy/bitcoin/tree/0.21.x-openrpc-backport
Bitcoin Core Version 0.20.2: https://github.com/satsfy/bitcoin/tree/0.20.x-openrpc-backport
Bitcoin Core Version 0.19.2: https://github.com/satsfy/bitcoin/tree/0.19.x-openrpc-backport
Bitcoin Core Version 0.18.2: https://github.com/satsfy/bitcoin/tree/0.18.x-openrpc-backport
Bitcoin Core Version 0.17.2: https://github.com/satsfy/bitcoin/tree/0.17.x-openrpc-backport (this particular version was manually written by AI because Core was missing general documentation on RPCs back then).
