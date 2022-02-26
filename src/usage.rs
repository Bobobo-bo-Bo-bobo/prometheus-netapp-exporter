use crate::constants;

pub fn show_version() {
    println!(
        "{} version {}
Copyright (C) 2022 by Andreas Maus <maus@ypbind.de>
This program comes with ABSOLUTELY NO WARRANTY.
    
{} is distributed under the Terms of the GNU General
Public License Version 3. (http://www.gnu.org/copyleft/gpl.html)
",
        constants::NAME,
        constants::VERSION,
        constants::NAME
    );
}

pub fn show_usage() {
    show_version();
    println!(
        "Usage: {} [-D|--debug] [-V|--version] -c <config>|--config=<config> [-h|--help] [-l <address>|--listen=<address>]

    -D                  Enable debug mode
    --debug

    -V                  Show version information
    --version

    -c <config>         Configuration file
    --config=<config>

    -h                  Show this help text
    --help

    -l <address>        Listen on <address> for scrape requests
    --listen=<address>  Default: {}

    -q                  Quiet operation. Only log warning
    --quiet             and error messages
",
        constants::NAME, constants::DEFAULT_PROMETHEUS_ADDRESS
    );
}
