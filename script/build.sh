#/bin/sh
forge="`dirname $0`/../../.."
forge=`realpath ${forge}`
layers=${forge}/base_layer
platform=docker
build="${layers}/build/${platform}"
machine=$1
toolchain=$2
action=${3:-build}

build() {
    ${docker_prefix} mkdir -p ${build}/${machine_basename}/${toolchain_basename}
    ${docker_prefix} cmake \
        -S ${layers}/${machine} \
        -B ${build}/${machine_basename}/${toolchain_basename} \
        -DCMAKE_BUILD_TYPE=Release \
        -Dhost_tools=${build}/host \
        --toolchain=${layers}/${toolchain}/riscv32_ilp32.cmake
    ${docker_prefix} cmake --build ${build}/${machine_basename}/${toolchain_basename}
}

collect() {
    for layer in `ls -d ${layers}/*/`
    do
        if [ -d ${layer}/$1 ]; then
            ls -d ${layer}$1/*/
        fi
    done
}

test() {
    ctest --test-dir ${build}/${machine_basename}/${toolchain_basename}
}

${docker_prefix} mkdir -p ${build}

case ${machine} in
    "ask")
        read -p "Machine (default "last"): " machine
        machine="${machine:-last}"
        ;;
esac

case ${toolchain} in
    "ask")
        read -p "Toolchain (default "last"): " toolchain
        toolchain="${toolchain:-last}"
        ;;
esac

case ${action} in
    "ask")
        read -p "Action (default "last"): " action
        action="${action:-last}"
        ;;
esac

case ${machine} in 
    "last")
        machines=`cat ${build}/last_machine`
        ;;
    "all")
        machines=$(collect "machine")
        echo "${machines}" > ${build}/last_machine
        ;;
    *)
        machines=${machine}
        echo "${machines}" > ${build}/last_machine
        ;;
esac

case ${toolchain} in 
    "last")
        toolchains=`cat ${build}/last_toolchain`
        ;;
    "all")
        toolchains=$(collect "toolchain")
        echo "${toolchains}" > ${build}/last_toolchain
        ;;
    *)
        toolchains=${toolchain}
        echo "${toolchains}" > ${build}/last_toolchain
        ;;
esac

case ${action} in 
    "last")
        action=`cat ${build}/last_action`
        ;;
    *)
        echo "${action}" > ${build}/last_action
        ;;
esac

for toolchain in ${toolchains}
do
    toolchain_basename=`basename ${toolchain}`
    set +e
    up=$(docker inspect -f '{{.State.Status}}' "${toolchain_basename}")
    set -e

    case ${up} in
        "running")
            echo "Container ${toolchain_basename} already running."
            ;;
        *)
            echo "Spinning up container ${toolchain_basename} from rvbl-${toolchain_basename}."
            docker run --interactive --tty --detach --name ${toolchain_basename} --volume ${forge}:${forge} rvbl-${toolchain_basename}
            ;;
    esac

    docker_prefix="docker exec --interactive --tty ${toolchain_basename}"

    ${docker_prefix} mkdir -p ${build}/host
    ${docker_prefix} cmake -S ${layers}/quintauris_internal/tool -B ${build}/host -DCMAKE_BUILD_TYPE=Release
    ${docker_prefix} cmake --build ${build}/host
    ${docker_prefix} ctest --test-dir ${build}/host
    
    for machine in ${machines}
    do
        machine_basename=`basename ${machine}`
        case ${action} in
            "build")
                build
                ;;
            "test")
                test 
                ;;
            "all")
                build
                test 
                ;;
        esac
    done
done