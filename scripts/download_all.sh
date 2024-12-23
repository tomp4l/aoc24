for i in $(seq 1 25); do
    curl 'https://adventofcode.com/2024/day/'$i'/input' \
        -H 'cookie: session='$SESSION \
        > input/day$i.txt
done
