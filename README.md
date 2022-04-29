# subdomain_ip_lister
Takes the output of dirbuster DNS and relates the subdomains to an IP, then tab delineates it for easy pasting into word tables.


# USAGE
subdomain_ip_lister /path/to/saved/gobuster/output

# Build instructions & install
```
rustup install stable
cd subdomain_ip_lister
cargo build
sudo cp target/debug/subdomain_ip_lister /usr/bin/
```
