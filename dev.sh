function do_start_postgres() {
    sudo systemctl start postgresql
}

function do_stop_postgres() {
    sudo systemctl stop postgresql
}

function do_run() {
    # assume postgres is running
    cargo run -r
}

function do_test() {
    cargo build -r
    cargo test -r
}

function do_build() {
    cargo build -r
}

if [ ! -z "$1" ]; then
    if [[ "$1" == 'postart' ]]; then
        do_start_postgres
    elif [[ "$1" == 'postop' ]]; then
        do_stop_postgres
    elif [[ "$1" == 'run' ]]; then
        do_run
    elif [[ "$1" == 'test' ]]; then
        do_test
    elif [[ "$1" == 'build' ]]; then
        do_build
    fi
else
    echo -e "choose an action [postart | postop | run | test | build]"
fi
