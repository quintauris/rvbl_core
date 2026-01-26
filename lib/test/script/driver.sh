#!/bin/sh
set -e

target=$1
machine=$2
image=$3
echo $image
shift 3
arguments="$@"

case $target in
    "qemu")
        if [ -x "$(command -v qemu-system-riscv32-$machine)" ]; then
            executable="qemu-system-riscv32-$machine"
        elif [ -x "$(command -v qemu-system-riscv32)" ]; then
            executable="qemu-system-riscv32"
        else
            echo "No qemu-system-riscv32 executable found."
            exit 1
        fi
        ${executable} ${arguments} -bios $image
        ;;
    "qemu.bin")
        if [ -x "$(command -v qemu-system-riscv32-$machine)" ]; then
            executable="qemu-system-riscv32-$machine"
        elif [ -x "$(command -v qemu-system-riscv32)" ]; then
            executable="qemu-system-riscv32"
        else
            echo "No qemu-system-riscv32 executable found."
            exit 1
        fi
        ${executable} ${arguments} -bios none -drive if=pflash,format=raw,unit=0,file.filename=${CMAKE_CURRENT_BINARY_DIR}/${image}
        ;;
    "jrun")
        if [ -x "$(command -v JRunExe)" ]; then
            JRunExe $arguments $image
            cat :tt
        else
            echo "No JRrun executable found."
            exit 1
        fi
        ;;
    "openocd")
        if [ -x "$(command -v openocd)" ]; then
            openocd ${arguments} -c 'load_image ${image}' -c resume -c 'sleep 3000' -c shutdown
        else
            echo "No JRrun executable found."
            exit 1
        fi
        ;;
    "vdk")
        if [ -x "$(command -v docker)" ]; then
            docker run ${arguments} --file $image
        else
            echo "No Docker executable found."
            exit 1
        fi
        ;;
    "vdk.bin")
        if [ -x "$(command -v docker)" ]; then
            docker run ${arguments} --file $image --nvm
        else
            echo "No Docker executable found."
            exit 1
        fi
        ;;
    "t32")
        if [ -x "$(command -v t32mriscv-qt)" ]; then
            rm -f output.txt
            t32mriscv-qt -s $arguments $image
            sleep 8
            cat output.txt
            rm -f output.txt
        else
            echo "No TRACE32 executable found."
            exit 1
        fi
        ;;
esac
