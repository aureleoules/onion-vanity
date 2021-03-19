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

## License
[MIT](https://github.com/aureleoules/onion-vanity/blob/master/LICENSE) - [Aurèle Oulès](https://github.com/aureleoules)