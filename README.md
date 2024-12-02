# Cuttlefish
Cuttlefish is a network mapper, NOT a port scanner. The difference is getting information about the connection between hosts for understanding the network topology, and not the information about what software runs on the host.

It works mainly by using the ICMP traceroutes for discovering paths and SSH/SNMP for fingerprinting.

This is a dogfood version, please expect bugs.

Found a bug (There should be plenty)? Please create an issue with logs from the `DEBUG_CUTTLEFISH=1` env variable, idealy with the known topology of the network.

Example output:
```
=Linux=       |--v           |--v           |--v           |--v           |--v           |--v
127.0.0.1------=Linux=       |=Linux=       |=Linux=       |=Linux=       |=Linux=       |=Linux=
               0.0.0.0--------10.0.7.1-------10.0.0.1      |10.0.5.1      |10.0.6.1      |10.0.16.1
               =Linux=        =Linux=        10.0.5.1      |=Linux=      ||--v           |=Linux=
               10.0.8.1-------|10.0.7.2      =Linux=       |10.0.5.2------|=Linux=       |10.0.16.20
               =Linux=        --v            10.0.4.1------|--v           |10.0.15.1     |=Linux=
               10.0.8.20      =Linux=       |--v           |=Linux=       |=Linux=       |10.0.16.21
               =Linux=        10.0.7.1-------=Linux=       |10.0.10.1     |10.0.15.2     |=Linux=
               10.0.8.21                     10.0.1.1      |10.0.11.1     |=Linux=       |10.0.16.22
               =Linux=                       10.0.4.1      |=Linux=       |10.0.14.1     |=Linux=
               10.0.8.22                     10.0.3.1      |10.0.12.1     |=Linux=       |10.0.16.23
               =Linux=                       10.0.5.1-------10.0.13.1     |10.0.14.2------
               10.0.8.23                     =Linux=        10.0.14.1    ||--v
                                             10.0.1.2       =Linux=      ||=Linux=
                                             10.0.2.2       10.0.13.1    ||10.0.2.1
                                             =Linux=        =Linux=      ||=Linux=
                                             10.0.4.1-------|10.0.13.20  ||10.0.2.2
                                             =Linux=        |=Linux=     ||
                                             10.0.4.2       |10.0.13.21  ||
                                             =Linux=        |=Linux=     ||
                                             0.0.0.0--------||10.0.13.22 ||
                                             =Linux=        ||=Linux=    ||
                                             10.0.3.1       ||10.0.13.23 ||
                                             =Linux=        ||=Linux=    ||
                                             10.0.5.1       ||10.0.13.24 ||
                                             =Linux=        ||=Linux=    ||
                                             10.0.5.2       ||10.0.13.25 ||
                                                            ||=Linux=    ||
                                                            ||10.0.14.1  ||
                                                            ||=Linux=    ||
                                                            ||10.0.14.2  ||
                                                            ||=Linux=    ||
                                                            ||10.0.5.2----|
                                                            |--v          |
                                                            |=Linux=      |
                                                            |10.0.10.1    |
                                                            |=Linux=      |
                                                            |10.0.11.1    |
                                                            --v           |
                                                            =Linux=       |
                                                            10.0.1.1      |
                                                            =Linux=       |
                                                            10.0.1.2-------
```
