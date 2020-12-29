# Arp Spoof

Ferramenta desenvolvida com o objetivo de colocar em prática os conhecimentos adquiridos estudando Rust e Redes.

## Como usar

```
$ cargo build
$ target/debug/arp_spoof -i <interface> -g <ip_gateway> -G <mac_gateway> -s <ip_host> -m <mac_host> -T <ip_alvo> -M <mac_alvo>


# Limpar mudanças no ARP table

$ target/debug/arp_spoof -i <interface> -g <ip_gateway> -G <mac_gateway> -s <ip_host> -m <mac_host> -T <ip_alvo> -M <mac_alvo> -r

```

## TODO

- Simplificar os argumentos de linha de comando


