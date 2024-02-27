# mullvad-closest-rs

A CLI tool to quickly find the best Mullvad Server. 

The easiest way to run it is by running

```sh 
cargo run --release
```

For getting help on how to run and configuring the other parameters:
```sh
cargo run --release -- --help
```

Although it supports Windows, Linux, I haven't tested those.

The result looks like below:
```
 country  | city               | latitude   | longitude    | hostname        | ipv4_addr       | ipv6_addr                 | pinged | latency
----------+--------------------+------------+--------------+-----------------+-----------------+---------------------------+--------+---------
 USA      | Atlanta, GA        | 33.753746  | -84.38633    | us-atl-ovpn-101 | 66.115.180.226  |                           | true   | 304.117
 USA      | Miami, FL          | 25.761681  | -80.191788   | us-mia-ovpn-102 | 146.70.183.66   |                           | true   | 304.998
 USA      | Miami, FL          | 25.761681  | -80.191788   | us-mia-wg-002   | 45.134.142.206  | 2a02:6ea0:cc1f:1::b61f    | true   | 305.016
 USA      | Miami, FL          | 25.761681  | -80.191788   | us-mia-ovpn-101 | 146.70.187.194  |                           | true   | 305.172
 USA      | Miami, FL          | 25.761681  | -80.191788   | us-mia-wg-001   | 45.134.142.219  | 2a02:6ea0:cc1f:2::b62f    | true   | 305.238
 USA      | Atlanta, GA        | 33.753746  | -84.38633    | us-atl-ovpn-103 | 66.115.180.228  |                           | true   | 306.356
 USA      | Miami, FL          | 25.761681  | -80.191788   | us-mia-wg-101   | 146.70.187.2    | 2a0d:5600:6:104::a01f     | true   | 306.576
 USA      | Miami, FL          | 25.761681  | -80.191788   | us-mia-wg-003   | 45.134.142.193  | 2a02:6ea0:cc1f::f001      | true   | 307.543
 USA      | Atlanta, GA        | 33.753746  | -84.38633    | us-atl-ovpn-104 | 66.115.180.229  |                           | true   | 308.305
```
The code is more or less inspired by ```Ch00k/mullvad-closest```
