#!/bin/bash

prev_ip="$(cat last_public_ip)"

while true; do
    current_ip=$(curl -s http://ipecho.net/plain)

    if [[ "$current_ip" != "$prev_ip" ]]; then
        timestamp=$(date +"[%d-%m-%Y] %H:%M:%S")
        echo "$timestamp > IP changed to $current_ip"
        #Save latest IP to file
        echo "$current_ip" > last_public_ip

        #Change DNS entry
        cflare dns update --zone mpw.sh af09631eb102873e10ccbf59ee23d2bd -c "$current_ip"
        cflare dns update --zone mpw.sh 8160a33da82fe7fcdbcfbeb8b8cdbd44 -c "$current_ip"

        #Save IP to variable
        prev_ip=$current_ip
    fi

    sleep 60
done
