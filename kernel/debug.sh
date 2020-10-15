#!/bin/bash

[ $# -ge 3 ] || { echo "Please use make dbg to run this script" && exit 1; }

kernel=$1
user1=$2
user2=$3
flags=$4

# generate small C program to ignore SIGINT
ignint=`mktemp XXXXXXX.c`
cat > $ignint <<EOF
#include <signal.h>
#include <unistd.h>

int main(int argc, char *argv[]) {
    sigset_t sigs;
    sigemptyset(&sigs);
    sigaddset(&sigs, SIGINT);
    sigprocmask(SIG_BLOCK, &sigs, 0);
    execvp(argv[1], argv + 1);
    return 1;
}
EOF

# build it
ignint_bin=`mktemp`
gcc -o build/ignint $ignint
rm $ignint

# run qemu with ignint to make ^C in gdb interrupt the guest
./build/ignint qemu-system-i386 $flags -kernel $kernel -initrd "$user1,$user2" -S -s &
pid=$!

# run gdb
tmp=`mktemp`
echo "target remote localhost:1234" >> $tmp
echo "display/i \$pc" >> $tmp
echo "b _start" >> $tmp
rust-gdb --tui $kernel --command=$tmp

# cleanup
kill $pid 2>/dev/null
rm $tmp
