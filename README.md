# Onion Vanity
Onion Vanity allows you to generate vanity onion addresses.

## Get started
```bash
$ cargo build --release
$ ./target/release/onion-vanity --help
...
USAGE:
    onion-vanity [FLAGS] [OPTIONS] --pattern <pattern>

FLAGS:
    -h, --help       Prints help information
    -v, --verbose    
    -V, --version    Prints version information

OPTIONS:
    -p, --pattern <pattern>    The pattern the onion address must start with
    -t, --threads <threads>    Number of threads [default: 4]
```

## Docker
```bash
$ docker run -it --rm aureleoules/onion-vanity --pattern "abcd" --threads 5
``` 

## Example
```bash
$ docker run -it --rm aureleoules/onion-vanity --pattern "vanity" --threads 16  
[2021-03-19T19:55:43Z INFO  onion_vanity] Started mining address: vanity  
[2021-03-19T20:06:39Z INFO  onion_vanity] Found onion address: vanity5xgb2hbe76g3trcr2tshykvtwh4iwkymuxzbmeci3xpcmcmqad.onion  
[2021-03-19T20:06:39Z INFO  onion_vanity] Private key: <private key>
```

Check it out!  
[http://vanity5xgb2hbe76g3trcr2tshykvtwh4iwkymuxzbmeci3xpcmcmqad.onion](http://vanity5xgb2hbe76g3trcr2tshykvtwh4iwkymuxzbmeci3xpcmcmqad.onion/)

## License
[MIT](https://github.com/aureleoules/onion-vanity/blob/master/LICENSE) - [Aurèle Oulès](https://github.com/aureleoules)