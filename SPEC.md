# Specification

## Modes

Modes determine what type of content the fuzzer will attempt to fuzz, for example: HTTP and DNS.

## Options

After selecting the mode, the fuzzer will need to be configured with additional options, there are multiple types of options:

* Input options

  Different inputs for fuzzing, in every input option you could insert placeholders for the fuzzed value. For example: URL, HTTP headers.

* Filtering Options

  Options for filtering relevant responses. For example: HTTP Status codes, HTTP Response size.

* General Options

  Configuring the wordlist, passing requests through a proxy, etc.

### Global Options

* `--delay duration` Delay between each request
* `--proxy string` Proxy to use for requests
* `--timeout duration` Request Timeout
* `-w, --wordlist string` Path to the wordlist
* `-t, --threads int` Number of threads to spawn
* `-p, --placeholder string` Placeholder to use (default: FUZZ)
* `-o, --output string`  Output file

### HTTP Options

#### General HTTP Options

* `-f, --follow-redirect` Follow redirects
* `-k, --insecure` Don't validate TLS certificates

#### Input

* `-u, --url string` The target URL
* `-a, --user-agent string` User-Agent header value
* `-c, --cookie string` Add a cookie to the request
* `-H, --header string` Add an aditional HTTP header
* `-m, --method string` HTTP Method to use (default "GET")
* `-U, --username string` Username for Basic Auth
* `-P, --password string` Password for Basic Auth

#### Filter Options

* `-s, --status string` Comma seperated list of statuses to show (Overwritten by `--status-hide`)
* `-S, --status-hide string` Comma seperated list of statuses to hide (Overwritten by `--status`)
* `-l, --content-length string` Comma seperated list of Content-Lengths and ranges to show (Overwritten by `--content-length-hide`)
* `-L, --content-length-hide` Comma seperated list of Content-Lengths and ranges to hide (Overwritten by `--content-length`)
* `-r, --regex string` Filter based on response bodies with a Regular Expression
* `--header-regex string` Filter based on response headers with a Regular Expression
