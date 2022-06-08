use std::net::ToSocketAddrs;

fn main()
{
    if let Some(host_port_combo) = std::env::args().skip(1).next()
    {
        for address in match host_port_combo.to_socket_addrs()
        {
            Ok(addresses) => addresses,
            Err(err) => return println!("Invalid host:port combo -> {}", err),
        }
        {
            println!("{:?}", address);
        }
    }
    else
    {
        println!("Usage: dns-lookup <hostname>:<port>");
    }
}
