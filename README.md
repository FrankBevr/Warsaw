### Set up a contract
0. `sudo service docker start`
1. `docker pull paritytech/contracts-ci-linux`
2. `docker run -it -p 9944:9944 -v /home/frank/Warsaw:/home/nonroot/Warsaw paritytech/contracts-ci-linux substrate-contracts-node --dev --unsafe-ws-external `
3. `docker ps`
4. `docker exec -it jovial_moore bash`
5. `cd /home/nonroot/Warsaw`
6. `cargo contract new warsaw_ink`
7. `cd warsaw_ink`
8. `cargo contract build`
9. `cargo contract upload --suri //Alice`
10. `cargo contract instantiate --suri //Alice --args true -x`
11. `cargo contract call --contract 5FfZRcR6iPuFQ7KDqNaMmsKn6v7EK4XPRGqkJDu8QXvrqENE --message get --suri //Alice`
12. `cargo contract call --contract 5FfZRcR6iPuFQ7KDqNaMmsKn6v7EK4XPRGqkJDu8QXvrqENE --message flip --suri //Alice -x`
13. `cargo contract call --contract 5FfZRcR6iPuFQ7KDqNaMmsKn6v7EK4XPRGqkJDu8QXvrqENE --message get --suri //Alice`

### Lets add a function
