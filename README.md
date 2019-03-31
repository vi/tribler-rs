Library and a command-line interface for Tribler REST API

(early version)

Currently only some commands are implemented:

```
$ tribler downloads list
 351.36 MiB    100%  2018-11-13-raspbian-stretch-lite.zip

$ tribler search begin test
   815.00 B test.log   magnet:?xt=urn:btih:98861d0ca6f44069a2c0e001e03bc95726b129de
   8.23 KiB test entry 102308   magnet:?xt=urn:btih:3430303738363637363637393233383537343830
     5.00 B test.txt   magnet:?xt=urn:btih:7de795fe731a31b2242e750d34974381ac2d53d8
...

$ tribler search completions hel
helloween
hellfire
hellblazer
```

License = Apache 2.0 or MIT.
