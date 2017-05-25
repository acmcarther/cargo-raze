#!/bin/sh
# Copyright 2014 The Rust Project Developers. See the COPYRIGHT
# file at the top-level directory of this distribution and at
# http://rust-lang.org/COPYRIGHT.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

msg() {
    echo "install: $1"
}

step_msg() {
    msg
    msg "$1"
    msg
}

warn() {
    echo "install: WARNING: $1"
}

err() {
    echo "install: error: $1"
    exit 1
}

need_ok() {
    if [ $? -ne 0 ]
    then
        err "$1"
    fi
}

need_cmd() {
    if command -v $1 >/dev/null 2>&1
    then msg "found $1"
    else err "need $1"
    fi
}

putvar() {
    local T
    eval T=\$$1
    eval TLEN=\${#$1}
    if [ $TLEN -gt 35 ]
    then
        printf "install: %-20s := %.35s ...\n" $1 "$T"
    else
        printf "install: %-20s := %s %s\n" $1 "$T" "$2"
    fi
}

valopt() {
    VAL_OPTIONS="$VAL_OPTIONS $1"

    local OP=$1
    local DEFAULT=$2
    shift
    shift
    local DOC="$*"
    if [ $HELP -eq 0 ]
    then
        local UOP=$(echo $OP | tr '[:lower:]' '[:upper:]' | tr '\-' '\_')
        local V="CFG_${UOP}"
        eval $V="$DEFAULT"
        for arg in $CFG_ARGS
        do
            if echo "$arg" | grep -q -- "--$OP="
            then
                val=$(echo "$arg" | cut -f2 -d=)
                eval $V=$val
            fi
        done
        putvar $V
    else
        if [ -z "$DEFAULT" ]
        then
            DEFAULT="<none>"
        fi
        OP="${OP}=[${DEFAULT}]"
        printf "    --%-30s %s\n" "$OP" "$DOC"
    fi
}

opt() {
    BOOL_OPTIONS="$BOOL_OPTIONS $1"

    local OP=$1
    local DEFAULT=$2
    shift
    shift
    local DOC="$*"
    local FLAG=""

    if [ $DEFAULT -eq 0 ]
    then
        FLAG="enable"
    else
        FLAG="disable"
        DOC="don't $DOC"
    fi

    if [ $HELP -eq 0 ]
    then
        for arg in $CFG_ARGS
        do
            if [ "$arg" = "--${FLAG}-${OP}" ]
            then
                OP=$(echo $OP | tr 'a-z-' 'A-Z_')
                FLAG=$(echo $FLAG | tr 'a-z' 'A-Z')
                local V="CFG_${FLAG}_${OP}"
                eval $V=1
                putvar $V
            fi
        done
    else
        if [ ! -z "$META" ]
        then
            OP="$OP=<$META>"
        fi
        printf "    --%-30s %s\n" "$FLAG-$OP" "$DOC"
     fi
}

flag() {
    BOOL_OPTIONS="$BOOL_OPTIONS $1"

    local OP=$1
    shift
    local DOC="$*"

    if [ $HELP -eq 0 ]
    then
        for arg in $CFG_ARGS
        do
            if [ "$arg" = "--${OP}" ]
            then
                OP=$(echo $OP | tr 'a-z-' 'A-Z_')
                local V="CFG_${OP}"
                eval $V=1
                putvar $V
            fi
        done
    else
        if [ ! -z "$META" ]
        then
            OP="$OP=<$META>"
        fi
        printf "    --%-30s %s\n" "$OP" "$DOC"
     fi
}

validate_opt () {
    for arg in $CFG_ARGS
    do
        isArgValid=0
        for option in $BOOL_OPTIONS
        do
            if test --disable-$option = $arg
            then
                isArgValid=1
            fi
            if test --enable-$option = $arg
            then
                isArgValid=1
            fi
            if test --$option = $arg
            then
                isArgValid=1
            fi
        done
        for option in $VAL_OPTIONS
        do
            if echo "$arg" | grep -q -- "--$option="
            then
                isArgValid=1
            fi
        done
        if [ "$arg" = "--help" ]
        then
            echo
            echo "No more help available for Configure options,"
            echo "check the Wiki or join our IRC channel"
            break
        else
            if test $isArgValid -eq 0
            then
                err "Option '$arg' is not recognized"
            fi
        fi
    done
}

absolutify() {
    FILE_PATH="${1}"
    FILE_PATH_DIRNAME="$(dirname ${FILE_PATH})"
    FILE_PATH_BASENAME="$(basename ${FILE_PATH})"
    FILE_ABS_PATH="$(cd ${FILE_PATH_DIRNAME} && pwd)"
    FILE_PATH="${FILE_ABS_PATH}/${FILE_PATH_BASENAME}"
    # This is the return value
    ABSOLUTIFIED="${FILE_PATH}"
}

msg "looking for install programs"
msg

need_cmd mkdir
need_cmd printf
need_cmd cut
need_cmd grep
need_cmd uname
need_cmd tr
need_cmd sed

CFG_SRC_DIR="$(cd $(dirname $0) && pwd)"
CFG_SELF="$0"
CFG_ARGS="$@"

HELP=0
if [ "$1" = "--help" ]
then
    HELP=1
    shift
    echo
    echo "Usage: $CFG_SELF [options]"
    echo
    echo "Options:"
    echo
else
    step_msg "processing $CFG_SELF args"
fi

# Check for mingw or cygwin in order to special case $CFG_LIBDIR_RELATIVE.
# This logic is duplicated from configure in order to get the correct libdir
# for Windows installs.
CFG_OSTYPE=$(uname -s)

case $CFG_OSTYPE in

    Linux)
        CFG_OSTYPE=unknown-linux-gnu
        ;;

    FreeBSD)
        CFG_OSTYPE=unknown-freebsd
        ;;

    DragonFly)
        CFG_OSTYPE=unknown-dragonfly
        ;;

    Darwin)
        CFG_OSTYPE=apple-darwin
        ;;

    MINGW*)
        # msys' `uname` does not print gcc configuration, but prints msys
        # configuration. so we cannot believe `uname -m`:
        # msys1 is always i686 and msys2 is always x86_64.
        # instead, msys defines $MSYSTEM which is MINGW32 on i686 and
        # MINGW64 on x86_64.
        CFG_CPUTYPE=i686
        CFG_OSTYPE=pc-windows-gnu
        if [ "$MSYSTEM" = MINGW64 ]
        then
            CFG_CPUTYPE=x86_64
        fi
        ;;

    MSYS*)
        CFG_OSTYPE=pc-windows-gnu
        ;;

# Thad's Cygwin identifers below

#   Vista 32 bit
    CYGWIN_NT-6.0)
        CFG_OSTYPE=pc-windows-gnu
        CFG_CPUTYPE=i686
        ;;

#   Vista 64 bit
    CYGWIN_NT-6.0-WOW64)
        CFG_OSTYPE=pc-windows-gnu
        CFG_CPUTYPE=x86_64
        ;;

#   Win 7 32 bit
    CYGWIN_NT-6.1)
        CFG_OSTYPE=pc-windows-gnu
        CFG_CPUTYPE=i686
        ;;

#   Win 7 64 bit
    CYGWIN_NT-6.1-WOW64)
        CFG_OSTYPE=pc-windows-gnu
        CFG_CPUTYPE=x86_64
        ;;
esac

OPTIONS=""
BOOL_OPTIONS=""
VAL_OPTIONS=""

if [ "$CFG_OSTYPE" = "pc-windows-gnu" ]
then
    CFG_LD_PATH_VAR=PATH
    CFG_OLD_LD_PATH_VAR=$PATH
elif [ "$CFG_OSTYPE" = "apple-darwin" ]
then
    CFG_LD_PATH_VAR=DYLD_LIBRARY_PATH
    CFG_OLD_LD_PATH_VAR=$DYLD_LIBRARY_PATH
else
    CFG_LD_PATH_VAR=LD_LIBRARY_PATH
    CFG_OLD_LD_PATH_VAR=$LD_LIBRARY_PATH
fi

flag uninstall "only uninstall from the installation prefix"
valopt destdir "" "set installation root"
opt verify 1 "verify that the installed binaries run correctly"
valopt prefix "/usr/local" "set installation prefix"
# NB This isn't quite the same definition as in `configure`.
# just using 'lib' instead of configure's CFG_LIBDIR_RELATIVE
valopt libdir "${CFG_DESTDIR}${CFG_PREFIX}/lib" "install libraries"
valopt mandir "${CFG_DESTDIR}${CFG_PREFIX}/share/man" "install man pages in PATH"

if [ $HELP -eq 1 ]
then
    echo
    exit 0
fi

step_msg "validating $CFG_SELF args"
validate_opt



# Template configuration.
# These names surrounded by '%%` are replaced by sed when generating install.sh

# Rust or Cargo
TEMPLATE_PRODUCT_NAME=%%TEMPLATE_PRODUCT_NAME%%
# rustc or cargo
TEMPLATE_VERIFY_BIN=%%TEMPLATE_VERIFY_BIN%%
# rustlib or cargo
TEMPLATE_REL_MANIFEST_DIR=%%TEMPLATE_REL_MANIFEST_DIR%%
# 'Rust is ready to roll.' or 'Cargo is cool to cruise.'
TEMPLATE_SUCCESS_MESSAGE=%%TEMPLATE_SUCCESS_MESSAGE%%

# OK, let's get installing ...

# Sanity check: can we run the binaries?
if [ -z "${CFG_DISABLE_VERIFY}" ]
then
    # Don't do this if uninstalling. Failure here won't help in any way.
    if [ -z "${CFG_UNINSTALL}" ]
    then
        msg "verifying platform can run binaries"
        export $CFG_LD_PATH_VAR="${CFG_SRC_DIR}/lib:$CFG_OLD_LD_PATH_VAR"
        "${CFG_SRC_DIR}/bin/${TEMPLATE_VERIFY_BIN}" --version 2> /dev/null 1> /dev/null
        if [ $? -ne 0 ]
        then
            err "can't execute rustc binary on this platform"
        fi
        export $CFG_LD_PATH_VAR="$CFG_OLD_LD_PATH_VAR"
    fi
fi

# Sanity check: can we can write to the destination?
msg "verifying destination is writable"
umask 022 && mkdir -p "${CFG_LIBDIR}"
need_ok "can't write to destination. consider \`sudo\`."
touch "${CFG_LIBDIR}/rust-install-probe" > /dev/null
if [ $? -ne 0 ]
then
    err "can't write to destination. consider \`sudo\`."
fi
rm -f "${CFG_LIBDIR}/rust-install-probe"
need_ok "failed to remove install probe"

# Sanity check: don't install to the directory containing the installer.
# That would surely cause chaos.
msg "verifying destination is not the same as source"
INSTALLER_DIR="$(cd $(dirname $0) && pwd)"
PREFIX_DIR="$(cd ${CFG_PREFIX} && pwd)"
if [ "${INSTALLER_DIR}" = "${PREFIX_DIR}" ]
then
    err "can't install to same directory as installer"
fi

# Using an absolute path to libdir in a few places so that the status
# messages are consistently using absolute paths.
absolutify "${CFG_LIBDIR}"
ABS_LIBDIR="${ABSOLUTIFIED}"

# The file name of the manifest we're going to create during install
INSTALLED_MANIFEST="${ABS_LIBDIR}/${TEMPLATE_REL_MANIFEST_DIR}/manifest"

# First, uninstall from the installation prefix.
# Errors are warnings - try to rm everything in the manifest even if some fail.
if [ -f "${INSTALLED_MANIFEST}" ]
then
    msg

    # Iterate through installed manifest and remove files
    while read p; do
        # The installed manifest contains absolute paths
        msg "removing $p"
        if [ -f "$p" ]
        then
            rm -f "$p"
            if [ $? -ne 0 ]
            then
                warn "failed to remove $p"
            fi
        else
            warn "supposedly installed file $p does not exist!"
        fi
    done < "${INSTALLED_MANIFEST}"

    # If we fail to remove $TEMPLATE_REL_MANIFEST_DIR below, then the
    # installed manifest will still be full; the installed manifest
    # needs to be empty before install.
    msg "removing ${INSTALLED_MANIFEST}"
    rm -f "${INSTALLED_MANIFEST}"
    # For the above reason, this is a hard error
    need_ok "failed to remove installed manifest"

    # Remove $TEMPLATE_REL_MANIFEST_DIR directory
    msg "removing ${ABS_LIBDIR}/${TEMPLATE_REL_MANIFEST_DIR}"
    rm -Rf "${ABS_LIBDIR}/${TEMPLATE_REL_MANIFEST_DIR}"
    if [ $? -ne 0 ]
    then
        warn "failed to remove ${TEMPLATE_REL_MANIFEST_DIR}"
    fi
else
    # There's no manifest. If we were asked to uninstall, then that's a problem.
    if [ -n "${CFG_UNINSTALL}" ]
    then
        err "unable to find installation manifest at ${CFG_LIBDIR}/${TEMPLATE_REL_MANIFEST_DIR}"
    fi
fi

# If we're only uninstalling then exit
if [ -n "${CFG_UNINSTALL}" ]
then
    echo
    echo "    ${TEMPLATE_PRODUCT_NAME} is uninstalled."
    echo
    exit 0
fi

# Create the installed manifest, which we will fill in with absolute file paths
mkdir -p "${CFG_LIBDIR}/${TEMPLATE_REL_MANIFEST_DIR}"
need_ok "failed to create ${TEMPLATE_REL_MANIFEST_DIR}"
touch "${INSTALLED_MANIFEST}"
need_ok "failed to create installed manifest"

msg

# Now install, iterate through the new manifest and copy files
while read p; do

    # Decide the destination of the file
    FILE_INSTALL_PATH="${CFG_DESTDIR}${CFG_PREFIX}/$p"

    if echo "$p" | grep "^lib/" > /dev/null
    then
        pp=`echo $p | sed 's/^lib\///'`
        FILE_INSTALL_PATH="${CFG_LIBDIR}/$pp"
    fi

    if echo "$p" | grep "^share/man/" > /dev/null
    then
        pp=`echo $p | sed 's/^share\/man\///'`
        FILE_INSTALL_PATH="${CFG_MANDIR}/$pp"
    fi

    # Make sure there's a directory for it
    umask 022 && mkdir -p "$(dirname ${FILE_INSTALL_PATH})"
    need_ok "directory creation failed"

    # Make the path absolute so we can uninstall it later without
    # starting from the installation cwd
    absolutify "${FILE_INSTALL_PATH}"
    FILE_INSTALL_PATH="${ABSOLUTIFIED}"

    # Install the file
    msg "installing ${FILE_INSTALL_PATH}"
    if echo "$p" | grep "^bin/" > /dev/null
    then
        install -m755 "${CFG_SRC_DIR}/$p" "${FILE_INSTALL_PATH}"
    else
        install -m644 "${CFG_SRC_DIR}/$p" "${FILE_INSTALL_PATH}"
    fi
    need_ok "file creation failed"

    # Update the manifest
    echo "${FILE_INSTALL_PATH}" >> "${INSTALLED_MANIFEST}"
    need_ok "failed to update manifest"

# The manifest lists all files to install
done < "${CFG_SRC_DIR}/lib/${TEMPLATE_REL_MANIFEST_DIR}/manifest.in"

msg

# Run ldconfig to make dynamic libraries available to the linker
if [ "$CFG_OSTYPE" = "unknown-linux-gnu" ]
    then
    ldconfig
    if [ $? -ne 0 ]
    then
        warn "failed to run ldconfig."
        warn "this may happen when not installing as root and may be fine"
    fi
fi

# Sanity check: can we run the installed binaries?
#
# As with the verification above, make sure the right LD_LIBRARY_PATH-equivalent
# is in place. Try first without this variable, and if that fails try again with
# the variable. If the second time tries, print a hopefully helpful message to
# add something to the appropriate environment variable.
if [ -z "${CFG_DISABLE_VERIFY}" ]
then
    msg "verifying installed binaries are executable"
    "${CFG_PREFIX}/bin/${TEMPLATE_VERIFY_BIN}" --version 2> /dev/null 1> /dev/null
    if [ $? -ne 0 ]
    then
        export $CFG_LD_PATH_VAR="${CFG_PREFIX}/lib:$CFG_OLD_LD_PATH_VAR"
        "${CFG_PREFIX}/bin/${TEMPLATE_VERIFY_BIN}" --version > /dev/null
        if [ $? -ne 0 ]
        then
            ERR="can't execute installed binaries. "
            ERR="${ERR}installation may be broken. "
            ERR="${ERR}if this is expected then rerun install.sh with \`--disable-verify\` "
            ERR="${ERR}or \`make install\` with \`--disable-verify-install\`"
            err "${ERR}"
        else
            echo
            echo "    Note: please ensure '${CFG_PREFIX}/lib' is added to ${CFG_LD_PATH_VAR}"
        fi
    fi
fi

echo
echo "    ${TEMPLATE_SUCCESS_MESSAGE}"
echo


