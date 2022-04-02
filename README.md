# HTTP Server Tester

This is a CLI test program for [HNU Cloud Computing Lab 2](https://github.com/1989chenguo/CloudComputingLabs/tree/master/Lab2).

## Install

For most student, you don't neet to rebuild this project.

We provide the release versions for main platforms. 

**Check your OS & goto [Release Page](https://github.com/LabCloudComputing/http-server-tester/releases) to download the correct zip.**

> If you cannot find the target zip, or the binary file cannot execute correctly, check [build](#build) introduction.

Unzip it, then you can find 2 folders: `config/` & `files/`, and 1 binary file: `http-server-tester`.

Move the whole program folder to anywhere you like, but **DON'T CHANGE** the relative path between files.

## Use

**ATTENTION**: Note that, unless otherwise specified, all relative paths of tester are relative to **the directory where you execute `./http-server-tester`**.

### Configure

There are 2 files in `config/`: `tester-config.json` & `log-config.yaml`.

`tester-config.json` defines commands and test items of the test program.

`log-config.yaml` defines how to output to the console & the log file.

What you need to do first is to modify the values of this keys in `tester-config.json`: 

- `directory` The directroy of your project,
- `build` The command you compile your project,
- `clean` The command you clean your project,
- `bin` The command you run your project,
- `server` The arguments for running your HTTP server. 

For example:

```json
{
    "directory": "/home/user/projects/http-server",
    "build": "make",
    "clean": "make clean",
    "bin": "./http-server",
    "server": {
        "ip": "127.0.0.1",
        "port": 8080,
        "threads": 8
    },
    ...
}
```

**ATTENTION**: `bin` is relative to `directory`.

> It's better to use absolute path for the key `directory`. 
> Relative path is OK, but don't use environment variables like `$HOME` or `~`.

> If you want to pass `build` or `clean`, just use a empty string `""`.

> For `server.ip`, use `"127.0.0.1"` instead of `"localhost"`. Because `ab` don't support it.

Tester will go to `/home/user/projects/http-server` and run the command `./http-server --ip 127.0.0.1 --port 8080 --threads 8`.

If you don't understand how the meanings of keys, check [Configure Tester](#configure-tester).

### Check Tools & Files

Install `curl` , `netcat(nc)` & `apache bench(ab)`, they are tools that will be called.

Files in the directory `./files/` will be compared with the HTTP response during testing.

They are specified in `./config/tester-config.json`.

### Run

There are some subcommands, **your most commonly used subcommand should be `run`.**

It has a argument `mode`, use `--mode basic` or `--mode advanced` to select tester work mode.

For example:

```bash
user@linux:~/http-server-tester$ ./http-server-tester run --mode advancd
```

Use `run` subcommand, tester will check `curl`, `nc` & `ab` tools , rebuild your projects, run your HTTP server and send requests to test.

You can also use other subcommands, like `build`, `dev`, to help you to develop the server.

> The different from `run` & `dev` is that `dev` subcommand won't rebuild your projects or try to run your HTTP server.

> The `build` part will not print any messages unless it builds failed.

> The output of your server will be printed to the console, but not the log file.

Use `http-server-tester --help` for more help information.

```bash
user@linux:~/http-server-tester$ ./http-server-tester --help
http-server-tester x.x.x
IceyBlackTea <IceyBlackTea@outlook.com>
A CLI test program for HNU Cloud Computing Lab 2, built with Rust.

USAGE:
    http-server-tester <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    build    Build the http server
    check    Check if tools have been installed
    clean    Clean the project directory
    dev      Test the running http server
    help     Print this message or the help of the given subcommand(s)
    run      Test the http server including rebuilding and starting
```

### Check Results

The output will be shown in console & be stored in the log file `./logs/tester.log`.

It will show the log level & messages.

You just need to pay attention to info and error levels.

For example:

```bash
user@linux:~/http-server-tester$ ./http-server-tester run --mode advancd
...
[WARN ] Trying to kill the HTTP Server...
[TRACE] The HTTP Server is stopped.
[INFO ] -------TESTER RESULTS------
[WARN ] HTTP test items: all 6, passes 3
[INFO ] Pipelining test items: all 2, passes 2
[WARN ] Proxy test items: all 2, passes 0
[INFO ] Perfermance test 1 times.
[INFO ] No.1: requests 100, concurrency 10, reqs/s 9273, time/req 0.108
[INFO ] -------TESTER RESULTS------
```

> Sorry, the log file will be replaced if you rerun tester.

## Build

You need to install [Rust](https://www.rust-lang.org/) toolchains.

In the `http-serve-teseter/`, `cargo build --release` for release version.

The excutable file will be generated in `./target/realse/`.

> Rust is hard but interesting. 😘

## More

### Configure Tester

#### tester-config.json

It is a JSON file, so please pay attention to the format when you modify it.

> If you are not familiar with JSON, check [background.md](https://github.com/1989chenguo/CloudComputingLabs/tree/master/Lab2/background.md) of Lab 2. 

It's a bit long, so please read it carefully.

##### root

| key | value type | description |
| --- | --- | --- |
| directroy | String | The directroy of your project |
| build | String | The command you compile your project |
| clean | String | The command you clean your project |
| bin | String | The command you run your project |
| server | Object | The arguments for running your HTTP server |
| items | Object | Test items |

##### server

| key | value type | description |
| --- | --- | --- |
| ip | String | The IP your server tend to bind |
| port | Integer | The port your server tend to bind |
| threads | Integer | The number of threads your server tend to use |

##### items

| key | value type | description |
| --- | --- | --- |
| wait_seconds | integer | The time for the tester to wait for your server to start |
| basic / advanced | Object | Test items of basic / advanced version |

> You can extend the waiting time appropriately if testing always starts before your server startups completely.

##### basic & advanced

| key | value type | description |
| --- | --- | --- |
| get | Array | Specific test items for GET |
| post | Array | Specific test items for POST |
| pipelining | Array | Specific test items for pipelining feature |
| prxoy | Array | Specific test items for proxy feature |
| performance | Array | Specific test items for perfermance |

> `pipelining`, `proxy` & `performance` are only tested in advanced version.

- `get[i]`
  - `path`: The url path that `curl` will request.
  - `file`: The file path of the correct result that will be compared with the response.
  
- `post[i]`
  - `path`: The url path that `curl` will request.
  - `payload`: The file path of the payload that will be sent with the request.
  - `file`: The file path of the correct result that will be compared with the response.
- `pipelining[i]`
  - A array contains paths that needs to access in one requests.
- `proxy[i]`
  - `host`: The proxy remote HTTP server.
  - `paths`: The array of url path that would be tested.

- `perfromance[i]`
  - `path`: The url path that `ab` will request.
  - `requests`: The `-n` argument of `ab`, number of requests to perform.
  - `concurrency`: The `-c` argument of `ab`, number of multiple requests to make at a time.

> If you modify the items, don't forget to move the correct file into files/.

#### log-config.yaml

If you modify the file incorrectly, the output format may be wrong, please try not to modify.

I use `log4rs` to print log messages & generate log files.

For more infomation, you can check [docs of log4rs](https://docs.rs/log4rs/latest/log4rs/).

### Why use Rust?

Just like it. 😎 Rust YYDS.

If you have any problems about this program, please write a issue.
